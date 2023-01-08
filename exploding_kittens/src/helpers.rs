use lazy_static::lazy_static;
use std::collections::HashMap;

use rand::{Rng, seq::SliceRandom};

use crate::strats::{CardData, Card, Hand, Strat, StratNope, StratCombo};

lazy_static! {
    pub static ref CARD_DATA:HashMap<Card, CardData> = HashMap::from([
        (Card::Defuse, CardData { freq: 6, combo: false, play: false, anti: false }),
        (Card::Kitten, CardData { freq: 4, combo: false, play: false, anti: false }),
        (Card::Nope, CardData { freq: 5, combo: false, play: false, anti: false }),
        (Card::Attack, CardData { freq: 4, combo: false, play: true, anti: true }),
        (Card::Skip, CardData { freq: 4, combo: false, play: true, anti: true  }),
        (Card::Favor, CardData { freq: 4, combo: false, play: true, anti: true }),
        (Card::Shuffle, CardData { freq: 4, combo: false, play: true, anti: true }),
        (Card::Future, CardData { freq: 5, combo: false, play: true, anti: false }),
        (Card::Cattermelon, CardData { freq: 4, combo: true, play: true, anti: false }),
        (Card::Beardcat, CardData { freq: 4, combo: true, play: true, anti: false }),
        (Card::Potatocat, CardData { freq: 4, combo: true, play: true, anti: false }),
        (Card::Rainbowcat, CardData { freq: 4, combo: true, play: true, anti: false }),
        (Card::Tacocat, CardData { freq: 4, combo: true, play: true, anti: false }),
    ]);
}

pub struct Helpers {}

impl Helpers 
{
    pub fn generate_deck() -> Vec<Card>
    {
        // generate full deck
        let mut all_cards:Vec<Card> = Vec::new();
        for (k,v) in CARD_DATA.iter()
        {
            if *k == Card::Defuse || *k == Card::Kitten { continue; }

            for _i in 0..v.freq
            {
                all_cards.push(*k);
            }
        }
        return all_cards;
    }

    pub fn request_nope_based_on_strategy(strat:StratNope) -> bool
    {
        let mut prob:f64 = 0.0;
        let mut rng = rand::thread_rng();
        match strat
        {
            StratNope::Random => { prob = 0.1; }
            StratNope::Never => { prob = 0.0; }
            StratNope::Rarely => { prob = 0.25; }
            StratNope::Sometimes => { prob = 0.5; }
            StratNope::Often => { prob = 0.75; }
            StratNope::Always => { prob = 1.0; }
            _ => { }
        }
        return rng.gen::<f64>() <= prob;
    }

    pub fn request_combo_based_on_strategy(strat:StratCombo) -> bool
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
        }
        return rng.gen::<f64>() <= prob;
    }

    pub fn remove_combo_cards(cards:&Hand) -> Hand
    {
        let mut arr:Hand = Vec::new();
        for v in cards.iter()
        {
            if CARD_DATA[v].combo { continue; }
            arr.push(*v);
        }
        return arr;
    }

    pub fn get_random_player_count() -> usize
    {
        let mut rng = rand::thread_rng();
        return rng.gen_range(2..=5);
    }

    pub fn is_direct_attack(num:usize, other_num:usize, card:Card, player_count:usize) -> bool
    {
        let next_player = (num + 1) % player_count;
        return next_player == other_num && Helpers::card_is_agressive(card);
    }

    pub fn opponent_will_nope(num:usize, card:Card, hands:&mut Vec<Hand>, strat:&Strat, direct_attack:bool) -> bool
    {
        if !hands[num].contains(&Card::Nope) { return false; }

        let will_nope:bool;
        let mut rng = rand::thread_rng();
        match strat.nope
        {
            StratNope::Random => { will_nope = rng.gen::<f64>() <= 0.1; }
            StratNope::Never => { will_nope = false; }
            StratNope::Rarely => { will_nope = rng.gen::<f64>() <= 0.25; }
            StratNope::Sometimes => { will_nope = rng.gen::<f64>() <= 0.5; }
            StratNope::Often => { will_nope = rng.gen::<f64>() <= 0.75; }
            StratNope::Always => { will_nope = true; }
            StratNope::OnlyIfSafe => {
                will_nope = hands[num].len() >= 5;
            }
            StratNope::OnlyDefuseless => {
                will_nope = !hands[num].contains(&Card::Defuse);
            }
            StratNope::Direct => {
                will_nope = direct_attack;
            }
            StratNope::DirectUnsafe => {
                will_nope = direct_attack && !hands[num].contains(&Card::Defuse);
            }
            StratNope::Wait => {
                will_nope = hands.len() <= 2;
            }
            StratNope::DeNope => { 
                will_nope = card == Card::Nope;
            }
            StratNope::DeNopeDirect => {
                will_nope = direct_attack && card == Card::Nope;
            }
        }

        return will_nope;
    }

    pub fn sort_descending(arr:&mut Vec<usize>)
    {
        arr.sort_by(|a, b| b.cmp(a));
    }

    pub fn get_actual_index(value:Card, arr:&Hand) -> usize
    {
        return arr.iter().position(|c| *c == value).unwrap();
    }

    pub fn get_random_player_order(player_count:usize) -> Vec<usize>
    {
        let mut rng = rand::thread_rng();
        let mut player_order:Vec<usize> = (0..player_count).collect();
        player_order.shuffle(&mut rng); 
        return player_order;
    }

    pub fn get_random_start_player(player_count:usize) -> usize
    {
        let mut rng = rand::thread_rng();
        return rng.gen_range(0..player_count);
    }

    pub fn get_anti_kitten_cards(cards:&Hand) -> Vec<Card>
    {
        let mut arr:Vec<Card> = Vec::new();
        for v in cards.iter()
        {
            if !CARD_DATA[v].anti { continue; }
            arr.push(*v);
        }
        return arr;
    }

    pub fn count_playable_cards(cards:&Hand) -> usize
    {
        let mut sum = 0;
        for v in cards.iter()
        {
            if !CARD_DATA[v].play { continue; }
            sum += 1;
        }
        return sum;
    }

    pub fn get_playable_cards(cards:&Hand) -> Vec<Card>
    {
        let mut arr:Vec<Card> = Vec::new();
        for v in cards.iter()
        {
            if !CARD_DATA[v].play { continue; }
            arr.push(*v);
        }
        return arr;
    }

    pub fn card_is_agressive(card:Card) -> bool
    {
        return CARD_DATA[&card].anti; // maybe not the best way to check agressiveness, but it's a good fit for now
    }

    pub fn to_string_list<T>(list:&Vec<T>) -> Vec<String> where T: std::fmt::Display
    {
        let mut arr:Vec<String> = Vec::new();
        for v in list.iter()
        {
            arr.push(v.to_string());
        }
        return arr;
    }
}