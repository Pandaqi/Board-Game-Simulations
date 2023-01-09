use rand::{Rng, seq::SliceRandom};

use crate::{config::{SimConfig, StratListStruct}, results::SimResults, helpers::{Helpers, CARD_DATA}, simulator::Simulator, strats::{Card, Strat, Hand, StratKitten, StratPlay, StratVictim, Combo}, combos::Combos, nope::Nope};

pub struct Debugger {
    pub enabled: bool
}

impl Debugger 
{
    fn print<T>(&self, txt:T) where T: std::fmt::Debug
    {
        if !self.enabled { return; }
        println!("{:#?}", txt);
    }

    fn println(&self)
    {
        println!("");
    }
}

pub struct Game {
    all_cards: Vec<Card>
}

#[derive(Eq, PartialEq, Debug)]
pub enum DrawResult {
    None,
    Death,
    Defuse
}

pub struct GameState {
    pub repeat_turns: usize,
    pub skip_draw: bool,
    pub saw_future: bool,
    pub will_draw_kitten: bool,
    pub cards_played: usize,
    pub prev_exploded: bool,
    pub exploded: Vec<bool>,
    pub prev_victim: Vec<usize>
}

impl GameState
{
    pub fn new() -> Self
    {
        Self {
            repeat_turns: 0,
            skip_draw: false,
            saw_future: false,
            will_draw_kitten: false,
            cards_played: 0,
            prev_exploded: false,
            exploded: Vec::new(),
            prev_victim: Vec::new()
        }
    }

    pub fn init(&mut self, player_count:usize)
    {
        self.exploded = vec![false; player_count];
        self.prev_victim = vec![10; player_count];
    }
    
    // resets everything that is NOT kept between turns
    // (those are: repeat_turns, prev_exploded)
    pub fn reset(&mut self)
    {
        self.skip_draw = false;
        self.saw_future = false;
        self.will_draw_kitten = false;
        self.cards_played = 0;
    }
}

impl Game
{
    pub fn new() -> Self
    {
        Self {
            all_cards: Vec::new(),
        }
    }

    pub fn setup(&mut self)
    {
        self.all_cards = Helpers::generate_deck();
    }

    pub fn play(&self, cfg:&SimConfig, res:&mut SimResults)
    {
        let mut player_count = cfg.player_count;
        if cfg.randomize_player_count { player_count = Helpers::get_random_player_count(); }

        let dealing_result = Game::create_player_hands(player_count, self.all_cards.clone());
        let mut deck = dealing_result.1;
        
        // NOTE: When a player dies, they are REMOVED from all these lists
        // This speeds up generation and lookup a lot (no need to skip inactive players all the time)
        // But players_alive is needed to remember the original player _number_ of whoever is left
        let mut hands = dealing_result.0;
        let mut strategies: Vec<Strat> = Game::generate_random_strategies(player_count, &res.options);
        let mut players_alive: Vec<usize> = (0..player_count).collect();

        let mut game_is_over: bool = false;
        let mut cur_player: usize = Helpers::get_random_start_player(player_count);
        
        let mut state = GameState::new();
        state.init(player_count);

        let debugger = Debugger { enabled: cfg.print_gameplay };

        debugger.println();
        debugger.print("== NEW GAME ==");
        debugger.print(&strategies);
        debugger.print(&hands);
        
        while !game_is_over
        {
            cur_player = cur_player % players_alive.len();

            debugger.println();
            debugger.print("= NEW TURN =");
            debugger.print("Current player?");
            debugger.print(cur_player);

            Game::play_cards(cur_player, &mut hands, &mut deck, &strategies, &mut state, &debugger);
            
            if state.skip_draw { 
                cur_player += 1;
                state.reset();
                debugger.print("Skipped draw");
                continue; 
            }

            let draw_result:DrawResult = Game::draw_card(cur_player, &mut hands, &mut deck, &debugger);
            match draw_result 
            {
                DrawResult::Death => { 
                    debugger.print("Draw => death");
                    Game::kill_player(cur_player, &mut hands, &mut players_alive, &mut strategies, &mut state); 
                }
                DrawResult::Defuse => { 
                    debugger.print("Draw => Defuse");
                    Game::put_back_kitten(cur_player, &hands, &mut deck, &strategies[cur_player], &state); 
                }
                _ => { }
            }

            game_is_over = players_alive.len() <= 1;
            state.reset();

            // repeat a turn by not changing the player
            state.prev_exploded = draw_result == DrawResult::Defuse;
            if game_is_over { state.repeat_turns = 0; }
            if state.repeat_turns > 0 { 
                state.repeat_turns -= 1;
                debugger.print("Repeating turn");
                debugger.print(state.repeat_turns);
                continue;
            }

            cur_player += 1;
        }

        let winning_player = players_alive[0];
        let winning_strategy:Strat = strategies[0];
        Simulator::save_results(res, winning_player, winning_strategy);
    }

    pub fn kill_player(num:usize, hands:&mut Vec<Hand>, players_alive:&mut Vec<usize>, strategies:&mut Vec<Strat>, state: &mut GameState)
    {
        hands.remove(num);
        strategies.remove(num);
        players_alive.remove(num);
        state.exploded.remove(num);
        state.prev_victim.remove(num);
    }

    fn play_cards(num:usize, hands:&mut Vec<Hand>, deck:&mut Vec<Card>, strategies:&Vec<Strat>, state:&mut GameState, debugger:&Debugger)
    {
        let strat = strategies[num];

        loop
        {
            if !Game::wants_to_continue_turn(num, hands, &strat, state) { break; }

            let mut cards_to_play = Game::pick_card_to_play(num, hands, &strat, state);
            if cards_to_play.len() <= 0 { break; }

            debugger.print("Chosen cards to play");
            debugger.print(&cards_to_play);

            // play the chosen cards
            while cards_to_play.len() > 0
            {
                let combo = cards_to_play.pop().unwrap();
                let success = Game::execute_card(num, hands, combo, deck, strategies, state);
                if !success { continue; }

                for _i in 0..combo.1
                {
                    let idx = hands[num].iter().position(|c| *c == combo.0).unwrap();
                    hands[num].remove(idx);
                }
                
                state.cards_played += 1;
            }

            debugger.print("Remaining hand");
            debugger.print(&hands[num]);
        }
    }

    pub fn wants_to_continue_turn(num:usize, hands:&Vec<Hand>, strat:&Strat, state:&GameState) -> bool
    {
        // determine if we want to keep playing cards (can't if we don't HAVE any cards)
        if Helpers::count_playable_cards(&hands[num]) <= 0 { return false; }

        // a general rule that every player will live by, regardless of strategy
        // if we know we will draw a kitten, and we have a card to stop it, DO THAT
        if state.saw_future && state.will_draw_kitten
        {
            return Helpers::get_anti_kitten_cards(&hands[num]).len() > 0;
        }

        let mut rng = rand::thread_rng();
        let mut keep_playing:bool = false;

        match strat.play
        {
            StratPlay::Random => { 
                keep_playing = rng.gen::<f64>() <= 0.5;
            }

            StratPlay::Never => { keep_playing = false; }
            StratPlay::One => { keep_playing = state.cards_played == 0; }
            StratPlay::All => { keep_playing = true; }
            StratPlay::OnlyAfterKitten => { 
                keep_playing = state.prev_exploded && state.cards_played <= 0;
            }
            StratPlay::NotIfSafe => {
                keep_playing = !hands[num].contains(&Card::Defuse) && state.cards_played <= 0;
            }
        }

        return keep_playing;
    }

    pub fn pick_card_to_play(num:usize, hands:&Vec<Hand>, strat:&Strat, state:&GameState) -> Vec<Combo>
    {
        // determine what we want to play
        let mut cards_to_play:Vec<Combo> = Vec::new();
        let mut playable_hand = Helpers::get_playable_cards(&hands[num]);
        let mut rng = rand::thread_rng();

        if playable_hand.len() <= 0 { return Vec::new(); }

        // the general rule: we know the future is bad, so prevent it
        if state.saw_future && state.will_draw_kitten
        {
            let arr = Helpers::get_anti_kitten_cards(&playable_hand);
            if arr.len() > 0 
            {
                let idx = playable_hand.iter().position(|c| *c == arr[0]).unwrap();
                cards_to_play.push((arr[0], 1));
                playable_hand.remove(idx);
            }
        }

        if playable_hand.len() <= 0 { return cards_to_play; }

        // COMBO CHECKER
        let mut play_combo:bool = false;
        let combo:Option<Combo> = Combos::get_combo(&playable_hand, strat);
        if combo.is_some() { play_combo = Combos::want_to_play_combo(combo.unwrap(), strat); }
        if play_combo { 
            cards_to_play.push(combo.unwrap()); 
            return cards_to_play;
        }
        
        // if we don't play a combo, those cards are useless now
        // onwards, _exclude_ those cards from the playable hand 
        playable_hand = Combos::remove_combo_cards(&playable_hand);
        if playable_hand.len() <= 0 { return cards_to_play; }

        // otherwise let our strategy decide
        // TO DO
        match strat.play
        {
            _ => { 
                let range = 0..playable_hand.len();
                let rand_idx = rng.gen_range(range);
                cards_to_play.push((playable_hand[rand_idx], 1));
                playable_hand.remove(rand_idx);
            }
        }

        return cards_to_play;
    }

    pub fn was_noped(num:usize, hands:&mut Vec<Hand>, card: Combo, strategies:&Vec<Strat>) -> bool
    {
        // random order is necessary, because ANYONE can nope, so no preference should be given
        let player_order:Vec<usize> = Helpers::get_random_player_order(hands.len());
        let mut num_nopes:usize = 0;

        let is_combo = card.1 > 1; // TO DO: use this for something?? A strategy that only nopes COMBOS

        loop 
        {
            let old_num_nopes = num_nopes;
            let nope_active:bool = num_nopes % 2 == 1;

            let card = if nope_active { Card::Nope } else { card.0 };
            for v in player_order.iter()
            {
                let idx = *v;
                if !nope_active && idx == num { continue; } // won't nope ourself

                let mut direct_attack = Helpers::is_direct_attack(num, idx, card, hands.len());
                if nope_active { direct_attack = idx == num; } // but we can un-nope ourself
                
                if Nope::opponent_will_nope(idx, card, hands, &strategies[idx], direct_attack) {
                    let nope_card_idx = hands[idx].iter().position(|c| *c == Card::Nope).unwrap();
                    hands[idx].remove(nope_card_idx);
                    num_nopes += 1;
                    break;
                }
            }

            if num_nopes == old_num_nopes { break; }
        }

        let ended_on_a_nope = num_nopes % 2 == 1;
        return ended_on_a_nope;
    }

    pub fn execute_card(num: usize, hands:&mut Vec<Hand>, combo:Combo, deck:&mut Vec<Card>, strategies:&Vec<Strat>, state:&mut GameState) -> bool
    {
        if !CARD_DATA[&combo.0].play { return false; }

        let mut rng = rand::thread_rng();

        let was_noped = Game::was_noped(num, hands, combo, strategies);
        if was_noped { return true; } // nope destroys the original card, even though it was never "played"

        let strat = strategies[num];

        // check combos
        if combo.1 == 2
        {
            Game::steal_card(num, hands, &strat, state, false);
        }

        if combo.1 == 3
        {
            Game::steal_card(num, hands, &strat, state, true);
        }

        // otherwise check the card on its own
        match combo.0
        {
            Card::Skip => { state.skip_draw = true; }
            Card::Shuffle => { 
                deck.shuffle(&mut rng); 
                state.saw_future = false;
                state.will_draw_kitten = false;
            }
            Card::Favor => {
                Game::steal_card(num, hands, &strat, state, false);
            }
            Card::Attack => { 
                state.skip_draw = true;
                if state.repeat_turns == 0 { state.repeat_turns = 1; }
                else { state.repeat_turns += 2; }
            }
            Card::Future => {
                let start = (deck.len() as i32 - 3).max(0) as usize;
                let end = deck.len();
                let range = start..end;
                if !range.is_empty()
                {
                    let last_three = deck.as_slice()[range].to_vec();
                    state.saw_future = true;
                    state.will_draw_kitten = last_three.contains(&Card::Kitten);
                }
            }
            _ => {}
        }

        return true;
    }

    pub fn steal_card(num:usize, hands:&mut Vec<Hand>, strat:&Strat, state:&mut GameState, need_to_request_card:bool)
    {
        // only consider players that are not us AND that have cards
        let player_count = hands.len();
        let mut other_players:Vec<usize> = (0..hands.len()).collect();
        other_players.remove(num);

        for i in (0..other_players.len()).rev()
        {
            let num_cards = hands[other_players[i]].len();
            if num_cards > 0 { continue; }
            other_players.remove(i);
        }

        let nobody_to_steal_from = other_players.len() <= 0;
        if nobody_to_steal_from { return; }

        let mut rng = rand::thread_rng();

        // NOTE: this variable is the REAL index of the player, not the idx in the "other_players" array
        let mut idx: usize = *other_players.choose(&mut rng).unwrap();
        let mut requested_card:Card = Helpers::get_random_card();

        if need_to_request_card && Nope::request_based_on_strategy(strat.nope)
        {
            requested_card = Card::Nope;
        }

        if need_to_request_card && Combos::request_based_on_strategy(strat.combo)
        {
            requested_card = Combos::pick_card_to_steal(&hands[num], strat.combo);
        }

        // TO DO: The PickOne and PickDiverse strats aren't 100% correct, because indices of players CHANGE as players are killed ...
        match strat.victim
        {
            StratVictim::Defuse => { requested_card = Card::Defuse; }
            StratVictim::DefuseProb => { 
                requested_card = Card::Defuse;

                for (k,v) in state.exploded.iter().enumerate()
                {
                    if !other_players.contains(&k) { continue; }
                    if *v { continue } // they are exploded, so they likely WON'T have a defuse card
                    idx = k; 
                    break;
                }
            }
            StratVictim::MostCards => { idx = Helpers::get_player_with_most_cards(other_players, &hands); }
            StratVictim::LeastCards => { idx = Helpers::get_player_with_least_cards(other_players, &hands); }
            
            StratVictim::SeatAfter => { 
                let new_idx = (num + 1) % player_count; 
                if other_players.contains(&new_idx) { idx = new_idx; }
            }

            StratVictim::SeatBefore => { 
                let new_idx = (num + player_count - 1) % player_count; 
                if other_players.contains(&new_idx) { idx = new_idx; }
            }
            
            StratVictim::PickOne => { 
                let prev_victim = state.prev_victim[num];
                let mut new_victim = prev_victim;
                if !other_players.contains(&new_victim) { new_victim = idx; }
                idx = new_victim;
                state.prev_victim[num] = new_victim;
            }
            StratVictim::PickDiverse => {
                let prev_victim = state.prev_victim[num];
                if !other_players.contains(&prev_victim) || idx == prev_victim 
                { 
                    for v in other_players.iter()
                    {
                        if *v == prev_victim { continue; }
                        idx = *v;
                        break;
                    }
                }
                state.prev_victim[num] = idx;
            }
            _ => {}
        }

        // get index of requested card, or do nothing if the other player doesn't have it
        let mut steal_idx = rng.gen_range(0..hands[idx].len());
        if need_to_request_card
        {
            if !hands[idx].contains(&requested_card) { return; }
            steal_idx = hands[idx].iter().position(|c| *c == requested_card).unwrap();
        }

        // finally, remove the card from our victim, add it to our hand
        let stolen_card = hands[idx].remove(steal_idx);
        hands[num].push(stolen_card);
    }

    pub fn draw_card(num:usize, hands:&mut Vec<Hand>, deck:&mut Vec<Card>, debugger:&Debugger) -> DrawResult
    {
        let card = deck.pop().unwrap();
        hands[num].push(card);

        debugger.print("Card drawn");
        debugger.print(card);

        if card != Card::Kitten { return DrawResult::None; }
        if !hands[num].contains(&Card::Defuse) { return DrawResult::Death; }

        hands[num].pop(); // remove kitten from hand again, is certainly in last place

        let defuse_card_idx = hands[num].iter().position(|c| *c == Card::Defuse).unwrap();
        hands[num].remove(defuse_card_idx);

        return DrawResult::Defuse;
    }

    // Deck is a stack: last item is top, and drawn first
    pub fn put_back_kitten(num:usize, hands:&Vec<Hand>, deck:&mut Vec<Card>, strats:&Strat, state:&GameState)
    {
        let mut rng = rand::thread_rng();
        let player_count = hands.len();
        let deck_size:i32 = deck.len() as i32;
        let range = 0..deck.len();
        if range.is_empty() 
        { 
            deck.insert(0, Card::Kitten); 
            return; 
        }

        let mut idx:i32 = rng.gen_range(range) as i32;
        match strats.kitten
        {
            StratKitten::Top => { idx = deck_size; }
            StratKitten::TopSecond => { idx = deck_size - 1; }
            StratKitten::TopFourth => { idx = deck_size - 3; }
            StratKitten::TopCond => { 
                let next_player = (num + 1) % player_count;
                if hands[next_player].len() <= 3 { idx = deck_size; }
            }
            StratKitten::Bottom => { idx = 0; }
            _ => { }
        }

        // take into account how many turns we still have to do
        let max_idx:i32 = (deck_size - (state.repeat_turns as i32)).max(0);

        // NOTE: we need to allow index of deck_size (instead of deck_size-1 as maximum)
        // as inserting at the end means using that index
        let final_idx = idx.clamp(0, max_idx) as usize;
        deck.insert(final_idx, Card::Kitten);
    }

    pub fn generate_random_strategies(player_count:usize, options:&StratListStruct) -> Vec<Strat>
    {
        let mut strats:Vec<Strat> = Vec::new();
        for _i in 0..player_count
        {
            strats.push(Strat::new_random(options));
        }
        return strats;
    }

    pub fn create_player_hands(player_count:usize, mut cards:Vec<Card>) -> (Vec<Hand>, Vec<Card>)
    {
        let mut hands:Vec<Hand> = Vec::new();
        let mut rng = rand::thread_rng();

        // give each player a defuse card
        for _i in 0..player_count
        {
            hands.push(vec![Card::Defuse]);
        }

        // hand out random cards
        let num_start_cards:usize = 7;
        for i in 0..player_count
        {
            for _j in 0..num_start_cards
            {
                let range = 0..cards.len();
                let rand_idx = rng.gen_range(range);
                hands[i].push(cards.remove(rand_idx));
            }
        } 

        // put defuse and kittens back into the deck, then shuffle
        let mut defuses_left = CARD_DATA[&Card::Defuse].freq - player_count;
        if player_count == 2 { defuses_left = 2; }

        let kittens_left = player_count - 1;
        for _i in 0..defuses_left
        {
            cards.push(Card::Defuse);
        }

        for _i in 0..kittens_left
        {
            cards.push(Card::Kitten);
        }

        cards.shuffle(&mut rng);

        return (hands, cards);
    }
}