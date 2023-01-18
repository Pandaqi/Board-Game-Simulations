use rand::Rng;

use crate::{strats::{Combo, Hand, Strat, Card, Strategy}, helpers::{CARD_DATA, Helpers}};

pub struct Combos {}

impl Combos
{
    pub fn is_combo_card(card:&Card) -> bool
    {
        return CARD_DATA[card].combo;
    }

    pub fn use_any_card(strat:&Strat) -> bool
    {
        let type_strat = *strat.get("combo_type").unwrap();
        if let Strategy::ComboType(tp) = type_strat
        {
            let prob = ((tp as usize) as f64) / 15.0;
            let mut rng = rand::thread_rng();
            return rng.gen::<f64>() <= prob;
        }
        return false;
    }

    pub fn wait_for_threes(strat:&Strat) -> bool
    {
        let pref_strat = *strat.get("combo_pref").unwrap();
        if let Strategy::ComboPref(tp) = pref_strat
        {
            let prob = ((tp as usize) as f64) / 15.0;
            let mut rng = rand::thread_rng();
            return rng.gen::<f64>() <= prob;
        }
        return false;
    }

    pub fn request_based_on_strategy(strat:&Strat) -> bool
    {
        let combo_strat = *strat.get("combo").unwrap();
        if let Strategy::Combo(tp) = combo_strat
        {
            let prob = ((tp as usize) as f64) / 15.0;
            let mut rng = rand::thread_rng();
            return rng.gen::<f64>() <= prob;
        }
        return false;
    }

    pub fn get_combo(cards:&Hand, strat:&Strat) -> Option<Combo>
    {
        let match_value = if Combos::wait_for_threes(strat) { 3 } else { 2 };
        let only_use_cat_cards = !Combos::use_any_card(strat);

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

    pub fn pick_card_to_steal(hand:&Hand, strat:&Strat) -> Card
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
        let wait_for_threes = Combos::wait_for_threes(strat);
        if wait_for_threes && combo.1 != 3 { return false; }
        return Combos::request_based_on_strategy(strat);
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