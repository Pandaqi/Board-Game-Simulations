#[cfg(test)]
mod tests {
    use crate::{config::SimConfig, game::State, helpers::Helpers};


    #[test]
    fn test_setup()
    {
        let mut cfg = SimConfig::new();

        let pairs = SimConfig::generate_unique_color_pairs(7, 1);
        assert_eq!(pairs.len(), 15);

        cfg.cards = SimConfig::generate_all_cards();
        assert_eq!(cfg.cards.len(), 105);

        cfg.options = SimConfig::generate_all_strats(&cfg);
        assert_eq!(cfg.options.contains_key("action"), true);

        cfg.player_count = 4;
        let state = State::new(&cfg);

        assert_eq!(state.score.len(), 4);
        assert_eq!(state.hands.len(), 4);
        assert_eq!(Helpers::count_cards_total_all_players(&state.hands), 4*4);
        assert_eq!(Helpers::count_cards_total(&state.hands[0]), 4); // number of starting cards; TO DO: add this to config instead?
        assert_eq!(state.deck.len(), 105-4*4);
    }

}