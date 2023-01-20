#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{config::SimConfig, game::{State, Card, Game}, helpers::Helpers};


    #[test]
    fn test_setup()
    {
        let mut cfg = SimConfig::new();

        let pairs = SimConfig::generate_unique_color_pairs(7, 1);
        assert_eq!(pairs.len(), 15);

        cfg.cards = SimConfig::generate_all_cards();
        assert_eq!(cfg.cards.len(), 105);

        cfg.double_strategy = true;
        cfg.options = SimConfig::generate_all_strats(&cfg);
        assert_eq!(cfg.options.contains_key("self_card"), true);

        cfg.double_strategy = false;
        cfg.options = SimConfig::generate_all_strats(&cfg);
        assert_eq!(cfg.options.contains_key("self_card"), false);

        cfg.player_count = 4;
        let state = State::new(&cfg);

        assert_eq!(state.score.len(), 4);
        assert_eq!(state.hands.len(), 4);
        assert_eq!(Helpers::count_cards_total_all_players(&state.hands), 4*4);
        assert_eq!(Helpers::count_cards_total(&state.hands[0]), 4); // number of starting cards; TO DO: add this to config instead?
        assert_eq!(state.deck.len(), 105-4*4);
    }

    #[test]
    fn test_scoring()
    {
        let mut cfg = SimConfig::new();
        cfg.test_setup(3);

        let mut state = State::new(&cfg);
        state.test_setup(3);

        let players_sorted = Game::score_players(&mut state);
        assert_eq!(players_sorted[0].1 >= players_sorted[1].1, true);
        assert_eq!(players_sorted[1].1 >= players_sorted[2].1, true);

        let hand = HashMap::from([(0,2),(3,4),(1,1)]);
        let top_card = Card::new(0,1,2);
        let num_matches = Helpers::count_matching_cards(&top_card, &hand);
        assert_eq!(num_matches, 3);

        let num_matches = Helpers::count_matching_colors(&top_card, &hand);
        assert_eq!(num_matches, 2);

        state.score = vec![4, 6, 5];
        assert_eq!(Helpers::get_player_with_highest_score(&state), 1);
        assert_eq!(Helpers::get_player_with_lowest_score(&state), 0);

        assert_eq!(Helpers::count_stacks_bigger_than(3, &hand), 1);
        assert_eq!(Helpers::count_stacks_smaller_than(2, &hand), 2);

        Game::find_winner(&mut state);
        assert_eq!(state.winner, 1);

        state.score = vec![10,6,2];
        Game::find_winner(&mut state);
        assert_eq!(state.winner, 0);

        let hand = HashMap::from([(0,2),(1,4),(2,1)]);
        let guaranteed_steal = Helpers::is_guaranteed_steal(&top_card, &hand);
        assert_eq!(guaranteed_steal, true);

    }

}