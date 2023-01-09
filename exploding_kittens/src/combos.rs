use rand::Rng;

use crate::{strats::{Combo, StratCombo, Hand, Strat, Card}, helpers::{CARD_DATA, Helpers}};

pub struct Combos {}

impl Combos
{
    pub fn is_combo_card(card:&Card) -> bool
    {
        return CARD_DATA[card].combo;
    }

    pub fn use_any_card(strat:StratCombo) -> bool
    {
        return strat == StratCombo::AllCards || strat == StratCombo::AllCardsThrees;
    }

    pub fn get_combo(cards:&Hand, strat:&Strat) -> Option<Combo>
    {
        let wait_for_threes = strat.combo == StratCombo::ThreesSometimes || strat.combo == StratCombo::ThreesAlways || strat.combo == StratCombo::AllCardsThrees;
        let match_value = if wait_for_threes { 3 } else { 2 };

        let only_use_cat_cards = !Combos::use_any_card(strat.combo);

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

    pub fn pick_card_to_steal(hand:&Hand, strat:StratCombo) -> Card
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
        let wait_for_threes = strat.combo == StratCombo::ThreesSometimes || strat.combo == StratCombo::ThreesAlways;
        if wait_for_threes && combo.1 != 3 { return false; }

        // TO DO: does it matter WHAT cards you're using for the combo? (Only cat cards, never special cards, etc.)

        return Combos::request_based_on_strategy(strat.combo);
    }

    pub fn request_based_on_strategy(strat:StratCombo) -> bool
    {
        let mut prob:f64 = 0.0;
        let mut rng = rand::thread_rng();
        match strat
        {
            StratCombo::Random => { prob = 0.33; }
            StratCombo::Never => { prob = 0.0; }
            StratCombo::Rarely => { prob = 0.25; }
            StratCombo::Sometimes | StratCombo::ThreesSometimes => { prob = 0.5; }
            StratCombo::Often => { prob = 0.75; }
            StratCombo::Always | StratCombo::ThreesAlways => { prob = 1.0; }
            StratCombo::AllCards | StratCombo::AllCardsThrees => { prob = 0.33; }
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