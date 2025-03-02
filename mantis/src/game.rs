use std::collections::HashMap;

use enum_iterator::Sequence;
use rand::{Rng, seq::SliceRandom};

use crate::{config::SimConfig, results::SimResults, simulator::Simulator, strats::{Ideas}, helpers::Helpers, display::Display};

pub type Hand = HashMap<Color, usize>;
pub type Deck = Vec<Card>;
pub type Color = u8;

#[derive(Clone, Debug, Sequence)]
pub struct Card { 
    pub color: Color,
    pub back1: Color,
    pub back2: Color,
}

impl Card 
{
    pub fn new(color: Color, back1: Color, back2: Color) -> Self
    {
        Self {
            color,
            back1,
            back2
        }
    }
}

pub struct State {
    pub print_gameplay: bool,

    pub turn_num: usize,
    pub screenshot_num: usize,
    pub winner: usize,
    pub player_count: usize,
    pub cur_player: usize,
    pub score_threshold: usize,
    pub hands: Vec<Hand>,
    pub deck: Vec<Card>,
    pub score: Vec<usize>,
    pub strategies: Vec<Ideas>,

    pub action_taken: Action,
    pub steal_success: bool,
    pub score_success: bool,
    pub last_player_ratings: Vec<i32>,
    pub last_victim: usize,
}

impl State 
{
    pub fn new(cfg:&SimConfig) -> Self
    {
        let player_count = cfg.player_count;
        let cur_player = Game::get_random_start_player(player_count);
        let (hands, deck) = Game::deal_cards(player_count, &mut cfg.cards.clone());
        let strategies = Game::determine_random_strategies(cfg);

        Self {
            print_gameplay: cfg.print_gameplay,
            winner: 0,
            turn_num: 0,
            screenshot_num: 0,
            player_count,
            cur_player,
            score_threshold: cfg.score_threshold,
            hands,
            deck,
            score: vec![0; cfg.player_count],
            strategies,
            steal_success: false,
            score_success: false,
            last_player_ratings: vec![-100; cfg.player_count],
            action_taken: Action::Score,
            last_victim: 0,
        }
    }

    pub fn test_setup(&mut self, player_count:usize)
    {
        self.deck = Vec::new();
        self.hands = Vec::new();
        for i in 0..(player_count as u8)
        {
            self.deck.push(Card::new(0,1,2));
            self.hands.push(
                HashMap::from([
                    (i, (i+1) as usize)
                ])
            );
        }
    }

    pub fn print<T>(&self, txt:T) where T: std::fmt::Debug
    {
        if !self.print_gameplay { return; }
        println!("{:#?}", txt);
    }

    pub fn println(&self)
    {
        if !self.print_gameplay { return; }
        println!("");
    }

    pub fn save_last_player_ratings(&mut self, list: &Vec<(usize, i32)>)
    {
        let mut vec:Vec<i32> = vec![-100; self.player_count];
        for (_k,v) in list.iter().enumerate()
        {
            vec[v.0] = v.1;
        }
        self.last_player_ratings = vec;
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Action
{
    Score,
    Steal
}

pub struct Game {}

impl Game
{
    pub fn play(cfg:&mut SimConfig, res:&mut SimResults, display: &mut Display)
    {
        SimConfig::randomize_player_count(cfg);

        let mut state = State::new(cfg);
        display.save_strategies_to_png(&mut state);
        display.save_strategies_to_png(&mut state);

        loop
        {
            Game::take_turn(&mut state, display);
            if Game::is_over(&mut state) { break; }
            Game::advance_player(&mut state);
        }

        Game::find_winner(&mut state);

        state.print("== GAME OVER ==");
        state.print("Winner?");
        state.print(state.winner);

        Simulator::save_results(res, state);
    }

    pub fn score(state:&mut State)
    {
        let card = Game::draw_top_card(state);
        let we_have_this_color = state.hands[state.cur_player].contains_key(&card.color);

        state.print("> Scoring");
        state.print(&card);
        state.print("Do we have it?");
        state.print(we_have_this_color);

        state.last_victim = state.cur_player;
        if we_have_this_color 
        {
            let how_many = *state.hands[state.cur_player].get(&card.color).unwrap() + 1;
            state.score[state.cur_player] += how_many;
            state.hands[state.cur_player].remove(&card.color);
            state.score_success = true;
            return;
        }

        state.hands[state.cur_player].insert(card.color, 1);
    }

    pub fn steal(state:&mut State, victim:usize)
    {
        let card = Game::draw_top_card(state);
        let they_have_this_color = state.hands[victim].contains_key(&card.color);

        state.print("> Stealing");
        state.print(&card);
        state.print("Victim?");
        state.print(victim);
        state.print("Their cards?");
        state.print(&state.hands[victim]);
        state.print("Do they have it?");
        state.print(they_have_this_color);

        state.last_victim = victim;
        if they_have_this_color 
        {
            let how_many = *state.hands[victim].get(&card.color).unwrap() + 1;
            *state.hands[state.cur_player].entry(card.color).or_insert(0) += how_many;
            state.hands[victim].remove(&card.color);
            state.steal_success = true;
            return;
        }

        state.hands[victim].insert(card.color, 1);
    }

    
    // @STRATEGY: here it takes a decision based on its strategies
    // Does it want to steal or score? And steal from whom?
    pub fn determine_action(state:&mut State) -> (Action, usize)
    {        
        // get scores, pick the highest one by default
        let player_scores = Game::score_players(state);
        state.save_last_player_ratings(&player_scores);

        let mut victim:usize = state.cur_player;
        let valid_victim_exists = player_scores.len() > 0;
        if valid_victim_exists { victim = player_scores.first().unwrap().0; }
        
        // if highest one is us, we score; otherwise steal
        let victim_is_us = victim == state.cur_player;
        let mut action:Action = if victim_is_us { Action::Score } else { Action::Steal };

        // check the single override strategy
        let mut rng = rand::thread_rng();
        let override_strat = Game::get_single_strat_points("override", state.cur_player, state);
        if override_strat == -4 { action = Action::Score; }
        if override_strat == 4 { action = if rng.gen::<f64>() <= 0.5 { Action::Score } else { Action::Steal } }
        
        // @EXCEPTION: check the guaranteed win!
        if Helpers::score_is_guaranteed_win(state) { action = Action::Score; }

        return (action, victim);
    }

    // @STRATEGY: here it takes a decision based on its strategies
    // We use a different strategy when scoring ourselves vs others
    pub fn score_player(player_num: usize, state:&mut State) -> i32
    {
        let mut points:i32 = 0;
        let top_card = state.deck.last().unwrap();
        let its_me = player_num == state.cur_player;

        if Helpers::is_guaranteed_steal(top_card, &state.hands[player_num])
        {
            points += Game::get_single_strat_points("guaranteed_steal", player_num, state);
        }

        let two_color_match = Helpers::count_matching_colors(top_card, &state.hands[player_num]) == 2;
        if two_color_match
        {
            points += Game::get_strat_points("match_two", player_num, state);
        }

        points += Game::get_strat_points("card_match", player_num, state)
                * Helpers::count_matching_cards(top_card, &state.hands[player_num]);

        points += Game::get_strat_points("color_match", player_num, state)
                * Helpers::count_matching_colors(top_card, &state.hands[player_num]);
        
        points += Game::get_strat_points("card", player_num, state)
                * Helpers::count_cards_total(&state.hands[player_num]) as i32;
         
        points += Game::get_strat_points("color", player_num, state) 
                * (state.hands[player_num].keys().len() as i32);
        
        points += Game::get_strat_points("biggest_stack", player_num, state)
                * Helpers::get_biggest_stack(&state.hands[player_num]);
        
        if Helpers::get_player_with_highest_score(state) == player_num
        {
            points += Game::get_strat_points("winner", player_num, state);

            let big_lead = Helpers::get_lead(state.score[player_num], state) >= 5;
            if big_lead
            {
                points += Game::get_single_strat_points("lead_score", player_num, state);
            }
        }

        let close_to_winning = state.score[player_num] >= (state.score_threshold - 3);
        if close_to_winning
        {
            points += Game::get_strat_points("near_win", player_num, state);
        }

        if Helpers::get_player_with_lowest_score(state) == player_num
        {
            points += Game::get_strat_points("loser", player_num, state);
        }

        if its_me 
        {
            points += Game::get_single_strat_points("offset", player_num, state);
        }

        let close_score = ((state.score[player_num] as i32) - (state.score[state.cur_player] as i32)).abs() <= 3;
        if close_score && !its_me
        {
            points += Game::get_single_strat_points("close_score", player_num, state);
        }

        let better_score = state.score[player_num] > state.score[state.cur_player];
        if better_score && !its_me
        {
            points += Game::get_single_strat_points("better_score", player_num, state);
        }

        points += Game::get_strat_points("small_stack", player_num, state)
                * Helpers::count_stacks_smaller_than(1, &state.hands[player_num]);
        
        points += Game::get_strat_points("big_stack", player_num, state)
                * Helpers::count_stacks_bigger_than(3, &state.hands[player_num]);

        return points;
    }

    pub fn get_single_strat_points(strat:&str, player_num: usize, state:&State) -> i32
    {
        let res : Option<&i32> = state.strategies[player_num].get(strat);
        if res.is_none() { return 0; }
        return *res.unwrap();
    }

    pub fn get_strat_points(strat:&str, player_num:usize, state:&State) -> i32
    {
        let its_me = player_num == state.cur_player;
        let mut name = Game::get_strat(strat, its_me);
        if !state.strategies[player_num].contains_key(&name)
        {
            name = Game::get_strat(strat, false);
        }
        return Game::get_single_strat_points(&name, player_num, state);
    }

    pub fn score_players(state:&mut State) -> Vec<(usize, i32)>
    {
        let mut arr:Vec<(usize, i32)> = Vec::new();
        let top_card = state.deck.last().unwrap().clone();

        for i in 0..state.player_count
        {
            if !Helpers::hand_matches_card_back(&top_card, &state.hands[i]) { continue; }

            let score = Game::score_player(i, state);
            arr.push((i, score));
        }

        arr.sort_by(|a, b| (b.1).partial_cmp(&a.1).unwrap());
        return arr;
    }
    
    pub fn get_strat(strat: &str, its_me:bool) -> String
    {
        if its_me { return "self_".to_owned() + strat; }
        return "other_".to_owned() + strat;
    }

    pub fn draw_top_card(state:&mut State) -> Card
    {
        return state.deck.pop().unwrap();
    }

    pub fn find_winner(state:&mut State)
    {
        let mut winner = 0;
        let mut highest_score = 0;
        let mut biggest_tank = 0;
        for i in 0..state.player_count
        {
            let better_option = state.score[i] > highest_score || (state.score[i] == highest_score && state.hands[i].len() > biggest_tank);
            if !better_option { continue; }

            highest_score = state.score[i];
            biggest_tank = state.hands[i].len();
            winner = i;
        }

        state.winner = winner;
    }

    pub fn take_turn(state:&mut State, display:&mut Display)
    {
        state.println();
        state.print("== NEW TURN ==");
        state.print("Current player");
        state.print(state.cur_player);

        let (action, victim) = Game::determine_action(state);
        state.action_taken = action.clone();
        display.save_gamestate_to_png(state, false);
        
        if action == Action::Score { Game::score(state); } else { Game::steal(state, victim); }
        display.save_gamestate_to_png(state, true);
    }

    pub fn advance_player(state:&mut State)
    {
        state.print("> End state");
        state.print("Cards?");
        state.print(&state.hands[state.cur_player]);
        state.print("Score?");
        state.print(state.score[state.cur_player]);

        let repeat_turn = state.player_count == 2 && state.steal_success; // a special rule for two player game
        if !repeat_turn
        {
            state.cur_player = (state.cur_player + 1) % state.player_count;
        }

        state.turn_num += 1;
        state.steal_success = false;
        state.score_success = false;
    }

    pub fn is_over(state:&mut State) -> bool
    {
        let regular_victory = state.score[state.cur_player] >= state.score_threshold;
        let empty_deck = state.deck.len() <= 0;
        return regular_victory || empty_deck;
    }

    pub fn get_random_player_count() -> usize
    {
        let mut rng = rand::thread_rng();
        return rng.gen_range(2..=6);
    }

    pub fn get_random_start_player(num:usize) -> usize
    {
        let mut rng = rand::thread_rng();
        return rng.gen_range(0..num);
    }

    pub fn determine_random_strategies(cfg:&SimConfig) -> Vec<Ideas>
    {
        let mut strats:Vec<Ideas> = Vec::new();
        for i in 0..cfg.player_count
        {
            strats.push(Game::get_random_strategy(i, cfg));
        }
        return strats;
    }

    pub fn get_random_strategy(player_num: usize, cfg:&SimConfig) -> Ideas
    {
        // @EXCEPTION: fixed strategy for player 0
        if cfg.track_per_player && player_num == 0 { return cfg.fixed.clone(); }

        let mut rng = rand::thread_rng();
        let mut strat:Ideas = HashMap::new();

        for (k,v) in cfg.options.iter()
        {
            strat.insert(k.clone(), *v.choose(&mut rng).unwrap());
        }

        // @EXCEPTION: if override is on, all other strategies don't matter
        // to prevent messing with the results, set everyhting else to 0 = pass
        // THIS IS NOT TRUE. Setting it to 0 _still_ messes with the results, adding way more entries for 0
        // This override strategy was a bad idea, just ignore it
        let override_val = strat.get("override");
        if override_val.is_some()
        {
            let is_override = override_val.unwrap().abs() == 4;
            if is_override
            {
                for (k,v) in strat.iter_mut()
                {
                    if k == "override" { continue; }
                    *v = 0;
                }
            }
        }

        return strat;
    }

    pub fn deal_cards(num:usize, deck:&mut Deck) -> (Vec<Hand>, Deck)
    {
        let mut rng = rand::thread_rng();
        deck.shuffle(&mut rng);

        let mut hands = Vec::new();
        let num_start_cards = 4;
        for i in 0..num
        {
            hands.push(HashMap::new());

            for _a in 0..num_start_cards
            {
                let card = deck.pop().unwrap();
                *hands[i].entry(card.color).or_insert(0) += 1;
            }
        }

        return (hands, deck.to_vec());
    }

}
