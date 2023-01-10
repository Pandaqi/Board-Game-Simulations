#[cfg(test)]
mod tests {
    use enum_iterator::all;

    use crate::{game::{Game, DrawResult, GameState, Debugger}, helpers::Helpers, strats::{StratPlay, StratKitten, Card, Strat, Hand, StratNope, Strategy, StratVictim}, simulator::Simulator, nope::Nope};

    #[test]
    fn test_rust_functionality()
    {
        let fields_auto = all::<Strategy>().collect::<Vec<_>>();
        println!("{:#?}", fields_auto);
        assert_eq!(fields_auto.contains(&Strategy::Victim(StratVictim::PickOne)), true);

        assert_eq!(matches!(Strategy::Victim(StratVictim::PickOne), Strategy::Victim(_)), true);
        assert_eq!(
            std::mem::discriminant(&Strategy::Victim(StratVictim::Defuse)) == 
            std::mem::discriminant(&Strategy::Victim(StratVictim::PickDiverse)
        ), true);
    }

    #[test]
    #[ignore]
    fn test_game_setup() {
        println!("{:#?}", Helpers::generate_deck());
        println!("{:#?}", Game::create_player_hands(4, Helpers::generate_deck()));

        let options = Simulator::setup().options;
        println!("{:#?}", Game::generate_random_strategies(3, &options));
        //assert_eq!(Game::create_player_hands(4), );
    }

    #[test]
    fn test_player_death() {
        let count:usize = 2;
        let options = Simulator::setup().options;

        let mut hands = Game::create_player_hands(count, Helpers::generate_deck()).0;

        let mut players_alive:Vec<usize> = (0..count).collect();
        let mut strategies = Game::generate_random_strategies(count, &options);
        let mut state = GameState::new();
        state.init(count);
        
        Game::kill_player(1, &mut hands, &mut players_alive, &mut strategies, &mut state);
        assert_eq!(hands.len(), 1);
        assert_eq!(players_alive.len(), 1);
        assert_eq!(strategies.len(), 1);
    }

    #[test]
    fn test_continue_turn()
    {
        let hands:Vec<Hand> = vec![vec![]];
        let options = Simulator::setup().options;
        let strat = Helpers::generate_random_strategy(&options);
        let state = GameState::new();
        assert_eq!(Game::wants_to_continue_turn(0, &hands, &strat, &state), false);
    }

    #[test]
    fn test_card_picking()
    {
        // sorting function
        let mut arr:Vec<usize> = vec![0,5,3,2,8];
        Helpers::sort_descending(&mut arr);
        assert_eq!(arr, vec![8,5,3,2,0]);

        // index manipulation
        let hand:Hand = vec![Card::Attack, Card::Nope, Card::Nope, Card::Skip, Card::Shuffle];
        assert_eq!(Helpers::get_actual_index(Card::Nope, &hand), 1);

        // general draw: should result in a NUMBER for the card index, but not modify the hand
        let hands:Vec<Hand> = vec![vec![Card::Attack]];
        let options = Simulator::setup().options;
        let state = GameState::new();
        let mut strat = Helpers::generate_random_strategy(&options);
        strat.insert("play".to_owned(), Strategy::Play(StratPlay::Random));
        assert_eq!(Game::pick_card_to_play(0, &hands, &strat, &state), vec![(Card::Attack, 1)]);
        assert_eq!(hands[0].len(), 1);

        

        // TO DO: more tests needed once we have more strategies for picking => especially COMBOS
        // => or should they have their own tests below?
    }

    #[test]
    fn test_card_execution()
    {
        let num = 0;
        let mut hands:Vec<Hand> = vec![vec![Card::Future]];
        let mut deck:Vec<Card> = vec![Card::Kitten];
        let options = Simulator::setup().options;
        let strats = Game::generate_random_strategies(3, &options);
        let mut state = GameState::new();

        // seeing future should update state and not crash when pile is smaller than 3
        Game::execute_card(num, &mut hands, (Card::Future, 1), &mut deck, &strats, &mut state);
        assert_eq!(state.saw_future, true);
        assert_eq!(state.will_draw_kitten, true);

        // skip should update state
        state = GameState::new();
        hands = vec![vec![Card::Skip]];
        Game::execute_card(num, &mut hands, (Card::Skip, 1), &mut deck, &strats, &mut state);
        assert_eq!(state.skip_draw, true);

        // favour should lead to us having one more card
        state = GameState::new();
        state.init(2);
        hands = vec![vec![Card::Favor], vec![Card::Skip]];
        Game::execute_card(num, &mut hands, (Card::Favor, 1), &mut deck, &strats, &mut state);
        assert_eq!(hands[num].len(), 2);
        assert_eq!(hands[1].len(), 0);

        // attack should update state
        // TO DO: later check for MULTIPLE attacks after each other, but how??
        state = GameState::new();
        hands = vec![vec![Card::Attack]];
        Game::execute_card(num, &mut hands, (Card::Attack, 1), &mut deck, &strats, &mut state);
        assert_eq!(state.repeat_turns, 1);
        assert_eq!(state.skip_draw, true);

        // test playable and unplayable cards
        hands = vec![vec![Card::Defuse]];
        assert_eq!(Game::execute_card(num, &mut hands, (Card::Defuse, 1), &mut deck, &strats, &mut state), false);
        hands = vec![vec![Card::Shuffle]];
        assert_eq!(Game::execute_card(num, &mut hands, (Card::Shuffle, 1), &mut deck, &strats, &mut state), true);

        // TO DO: test shuffle, should reset some state variables (about if you're expecting a kitten and stuff)

        // test different card indices / locations
        state = GameState::new();
        hands = vec![vec![Card::Nope, Card::Skip]];
        Game::execute_card(num, &mut hands, (Card::Skip, 1), &mut deck, &strats, &mut state);
        assert_eq!(state.skip_draw, true);

    }

    #[test]
    fn test_card_stealing()
    {
        let count:usize = 2;
        let mut hands:Vec<Hand> = vec![vec![], vec![]];
        let options = Simulator::setup().options;
        let strat = Helpers::generate_random_strategy(&options);
        let mut state = GameState::new();
        state.init(count);

        // should do nothing (and not crash) if nobody to steal from
        Game::steal_card(0, &mut hands, &strat, &mut state, false);
        assert_eq!(hands[0].len(), 0);

        // regular steal
        hands = vec![vec![], vec![Card::Nope]];
        Game::steal_card(0, &mut hands, &strat, &mut state, false);
        assert_eq!(hands[0], vec![Card::Nope]);

        // TO DO: test specific strategies / combo stealing
        // (fourth parameter = true) for specific card requesting
    }

    #[test]
    fn test_card_drawing()
    {
        let mut hands:Vec<Hand> = vec![vec![Card::Defuse]];
        let mut deck:Vec<Card> = vec![Card::Kitten];
        let debugger = Debugger { enabled: true };

        // defuse: should mean we lose the defuse card and top card of deck is gone
        assert_eq!(Game::draw_card(0, &mut hands, &mut deck, &debugger), DrawResult::Defuse);
        assert_eq!(hands[0].len(), 0);
        assert_eq!(deck.len(), 0);

        // no defuse card? always death
        hands = vec![vec![Card::Attack, Card::Nope]];
        deck = vec![Card::Kitten];
        assert_eq!(Game::draw_card(0, &mut hands, &mut deck, &debugger), DrawResult::Death);

        // anything else: you get an extra card and that's it
        hands = vec![Vec::new()];
        deck = vec![Card::Skip, Card::Shuffle];

        assert_eq!(Game::draw_card(0, &mut hands, &mut deck, &debugger), DrawResult::None);
        assert_eq!(hands[0], vec![Card::Shuffle]);
        assert_eq!(deck, vec![Card::Skip]);
    }

    #[test]
    fn test_card_putback_kitten()
    {
        let mut hands:Vec<Hand> = vec![vec![]];
        let mut deck:Vec<Card> = vec![Card::Nope, Card::Attack, Card::Skip];
        let options = Simulator::setup().options;
        let mut strat = Helpers::generate_random_strategy(&options);
        strat.insert("kitten".to_owned(), Strategy::Kitten(StratKitten::Top));
        let state = GameState::new();

        // tests top strategy
        Game::put_back_kitten(0, &hands, &mut deck, &strat, &state);
        assert_eq!(*deck.last().unwrap(), Card::Kitten);

        // tests empty deck
        deck = vec![];
        Game::put_back_kitten(0, &hands, &mut deck, &strat, &state);        
        assert_eq!(deck, vec![Card::Kitten]);

        // tests top cond strategy
        hands = vec![vec![Card::Attack], vec![Card::Attack]];
        deck = vec![Card::Nope, Card::Attack, Card::Skip];
        strat.insert("kitten".to_owned(), Strategy::Kitten(StratKitten::TopCond));
        Game::put_back_kitten(0, &hands, &mut deck, &strat, &state);
        assert_eq!(*deck.last().unwrap(), Card::Kitten);
    }

    #[test]
    fn test_nope_mechanism()
    {
        let num:usize = 0;
        let opponent_num = 1;
        let card = Card::Attack;
        let mut hands:Vec<Hand> = vec![vec![Card::Attack], vec![Card::Nope]];
        let options = Simulator::setup().options;
        let mut strat = Helpers::generate_random_strategy(&options);
        strat.insert("nope".to_owned(), Strategy::Nope(StratNope::Always));

        // => per player nope decisions
        // direct attack calculations
        assert_eq!(Helpers::is_direct_attack(num, opponent_num, card, 2), true);

        // basic nope decision 
        let direct_attack = true;
        assert_eq!(Nope::opponent_will_nope(opponent_num, card, &mut hands, &strat, direct_attack), true);

        // no nope cards
        hands[1] = Vec::new();
        assert_eq!(Nope::opponent_will_nope(opponent_num, card, &mut hands, &strat, direct_attack), false);

        // => general nope loop
        // player order
        println!("Random player order for 4 players");
        println!("{:#?}", Helpers::get_random_player_order(4));

        // insta-nope by other player
        hands = vec![vec![Card::Attack], vec![Card::Nope]];
        let mut strats = vec![strat.clone(), strat.clone()];
        let nope_result:bool = Game::was_noped(num, &mut hands, (Card::Attack, 1), &strats);
        assert_eq!(hands[1].len(), 0);
        assert_eq!(nope_result, true);

        // double nope
        // (both lose that card, but the end result is false = no noping)
        hands = vec![vec![Card::Attack, Card::Nope], vec![Card::Nope]];
        strats[0].insert("nope".to_owned(), Strategy::Nope(StratNope::DeNopeDirect));
        let nope_result:bool = Game::was_noped(num, &mut hands, (Card::Attack, 1), &strats);
        assert_eq!(nope_result, false);
        assert_eq!(hands[0].len(), 1);
        assert_eq!(hands[1].len(), 0);

    }

    #[test]
    fn test_combo_mechanism()
    {

    }

}