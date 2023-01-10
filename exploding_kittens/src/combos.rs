use rand::Rng;

use crate::{strats::{Combo, StratCombo, Hand, Strat, Card, Strategy}, helpers::{CARD_DATA, Helpers}};

pub struct Combos {}

impl Combos
{
    pub fn is_combo_card(card:&Card) -> bool
    {
        return CARD_DATA[card].combo;
    }

    pub fn use_any_card(strat:Strategy) -> bool
    {
        return strat == Strategy::Combo(StratCombo::AllCards) 
            || strat == Strategy::Combo(StratCombo::AllCardsThrees);
    }

    pub fn wait_for_threes(strat:Strategy) -> bool
    {
        return strat == Strategy::Combo(StratCombo::ThreesSometimes) 
            || strat == Strategy::Combo(StratCombo::ThreesAlways)
            || strat == Strategy::Combo(StratCombo::AllCardsThrees);
    }

    pub fn get_combo(cards:&Hand, strat:&Strat) -> Option<Combo>
    {
        let combo_strat = *strat.get("combo").unwrap();
        let match_value = if Combos::wait_for_threes(combo_strat) { 3 } else { 2 };

        let only_use_cat_cards = !Combos::use_any_card(combo_strat);

        let freqs = Helpers::create_frequency_map(cards);
        let mut choice:Option<Combo> = None;

        // HashMap is randomly iterated, so this picks randomly
        for (k,v) in freqs.iter()
        {
            // lesser than is crucial, otherwise we miss all matches where we have MANY cards of the same type
            if *v < match_value { continue; } 
            if !Combos::is_combo_card(k) && only_use_cat_cards { continue; }
            choice = Some((*k, *v));
            break;
        }

        return choice;
    }

    pub fn pick_card_to_steal(hand:&Hand, strat:Strategy) -> Card
    {
        let only_use_cat_cards = !Combos::use_any_card(strat);
        let freqs = Helpers::create_frequency_map(hand);
        let mut best_option:Card = Card::Defuse;
        let mut best_freq:usize = 0;

        for (k,v) in freqs.iter()
        {
            if !Combos::is_combo_card(k) && only_use_cat_cards { continue; }
            if *v <= best_freq { continue; }
            best_option = *k;
            best_freq = *v;
        }

        return best_option;
    }

    pub fn want_to_play_combo(combo:Combo, strat:&Strat) -> bool
    {
        let combo_strat = *strat.get("combo").unwrap();
        let wait_for_threes = Combos::wait_for_threes(combo_strat);
        if wait_for_threes && combo.1 != 3 { return false; }

        // TO DO: does it matter WHAT cards you're using for the combo? (Only cat cards, never special cards, etc.)

        return Combos::request_based_on_strategy(combo_strat);
    }

    pub fn request_based_on_strategy(strat:Strategy) -> bool
    {
        let mut prob:f64 = 0.0;
        let mut rng = rand::thread_rng();
        match strat
        {
            Strategy::Combo(StratCombo::Random) => { prob = 0.33; }
            Strategy::Combo(StratCombo::Never) => { prob = 0.0; }
            Strategy::Combo(StratCombo::Rarely) => { prob = 0.25; }
            Strategy::Combo(StratCombo::Sometimes) | Strategy::Combo(StratCombo::ThreesSometimes) => { prob = 0.5; }
            Strategy::Combo(StratCombo::Often) => { prob = 0.75; }
            Strategy::Combo(StratCombo::Always) | Strategy::Combo(StratCombo::ThreesAlways) => { prob = 1.0; }
            Strategy::Combo(StratCombo::AllCards) | Strategy::Combo(StratCombo::AllCardsThrees) => { prob = 0.33; }
            _ => {}
        }
        return rng.gen::<f64>() <= prob;
    }

    pub fn remove_combo_cards(cards:&Hand) -> Hand
    {
        let mut arr:Hand = Vec::new();
        for v in cards.iter()
        {
            if Combos::is_combo_card(v) { continue; }
            arr.push(*v);
        }
        return arr;
    }
}