use enum_iterator::all;
use lazy_static::lazy_static;
use std::collections::HashMap;

use rand::{Rng, seq::{SliceRandom, IteratorRandom}};

use crate::{strats::{CardData, Card, Hand, Strat, StratList, Strategy, StratAnswer, StratFuture}, combos::Combos};

lazy_static! {
    pub static ref CARD_DATA:HashMap<Card, CardData> = HashMap::from([
        (Card::Defuse, CardData { freq: 6, combo: false, play: false, anti: false }),
        (Card::Kitten, CardData { freq: 4, combo: false, play: false, anti: false }),
        (Card::Nope, CardData { freq: 5, combo: false, play: false, anti: false }),
        (Card::Attack, CardData { freq: 4, combo: false, play: true, anti: true }),
        (Card::Skip, CardData { freq: 4, combo: false, play: true, anti: true  }),
        (Card::Favor, CardData { freq: 4, combo: false, play: true, anti: false }),
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
    pub fn create_enum_match_list(full:&Vec<Strategy>, key:Strategy) -> Vec<Strategy>
    {
        let mut arr:Vec<Strategy> = Vec::new();
        for v in full.iter()
        {
            if std::mem::discriminant(v) != std::mem::discriminant(&key) { continue; }
            arr.push(v.clone());
        }
        return arr;
    }

    pub fn get_all_possible_cards() -> Vec<Card>
    {
        return all::<Card>().collect::<Vec<_>>();
    }

    pub fn count_total_cards(cards:&Vec<Hand>) -> usize
    {
        return cards.iter().map(Vec::len).sum();
    }

    // For each card type, we only save it in the new array once in a while
    // The "a while" is equal to how frequent a card is (so more frequent cards are skipped more, compensating)
    pub fn compensate_for_card_frequency(cards:&Vec<Card>) -> Vec<Card>
    {
        let mut map:HashMap<Card, usize> = HashMap::new();
        let mut arr:Vec<Card> = Vec::new();
        for v in cards.iter()
        {
            *map.entry(*v).or_insert(0) += 1;
            if map.get(v).unwrap() % CARD_DATA[v].freq != 1 { continue; }
            arr.push(*v);
        }
        return arr;
    }

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

    pub fn get_random_card() -> Card
    {
        let mut rng = rand::thread_rng();
        let mut card = *CARD_DATA.keys().choose(&mut rng).unwrap();
        if card == Card::Kitten { card = Card::Defuse; }
        return card;
    }

    pub fn get_player_with_most_cards(valid_players:Vec<usize>, hands:&Vec<Hand>) -> usize
    {
        let mut num:usize = 0;
        let mut max:usize = 0;
        for (k,v) in hands.iter().enumerate()
        {
            if !valid_players.contains(&k) { continue; }
            if v.len() <= max { continue; }
            max = v.len();
            num = k;
        }
        return num;
    }

    pub fn get_player_with_least_cards(valid_players:Vec<usize>, hands:&Vec<Hand>) -> usize
    {
        let mut num:usize = 0;
        let mut min:usize = 1000;
        for (k,v) in hands.iter().enumerate()
        {
            if !valid_players.contains(&k) { continue; }
            if v.len() >= min { continue; }
            min = v.len();
            num = k;
        }
        return num;
    }

    pub fn create_frequency_map(arr:&Vec<Card>) -> HashMap<Card, usize>
    {
        let mut map = HashMap::new();
        for n in arr {
            *map.entry(*n).or_insert(0) += 1;
        }
        return map;
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

    pub fn will_play_future(my_hand:&Hand, strat:&Strat) -> bool
    {
        let future_strat = *strat.get("future").unwrap();
        let mut rng = rand::thread_rng();
        let mut response:bool = false;
        match future_strat
        {
            Strategy::Future(StratFuture::Random) => { response = rng.gen::<f64>() <= 0.5; }
            Strategy::Future(StratFuture::Never) => { response = false; }
            Strategy::Future(StratFuture::Rarely) => { response = rng.gen::<f64>() <= 0.25; }
            Strategy::Future(StratFuture::InstaChange) => {
                response = Helpers::get_anti_kitten_cards(my_hand).len() > 0;
            }
            Strategy::Future(StratFuture::Defuseless) => {
                response = !my_hand.contains(&Card::Defuse);
            }
            Strategy::Future(StratFuture::Often) => { response = rng.gen::<f64>() <= 0.75; }
            Strategy::Future(StratFuture::Always) => { response = true; }
            _ => {}
        }
        return response;
    }

    pub fn will_answer_previous_explosion(num:usize, hands:&Vec<Hand>, strat:&Strat) -> bool
    {
        let answer_strat = *strat.get("answer").unwrap();
        let mut rng = rand::thread_rng();
        let mut response:bool = false;
        match answer_strat
        {
            Strategy::Answer(StratAnswer::Random) => { response = rng.gen::<f64>() <= 0.5; }
            Strategy::Answer(StratAnswer::Ignore) => { response = false; }
            Strategy::Answer(StratAnswer::Defuseless) => {
                response = !hands[num].contains(&Card::Defuse);
            }
            Strategy::Answer(StratAnswer::FewPlayers) => {
                response = hands.len() <= 3;
            }
            Strategy::Answer(StratAnswer::Always) => { response = true; }
            _ => {}
        }
        return response;
    }

    pub fn sort_descending(arr:&mut Vec<usize>)
    {
        arr.sort_by(|a, b| b.cmp(a));
    }

    pub fn get_index(value:Card, arr:&Hand) -> usize
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

    pub fn generate_random_strategy(options:&StratList) -> Strat
    {
        let mut rng = rand::thread_rng();
        let mut strat:Strat = HashMap::new();
        for (k,v) in options.iter()
        {
            strat.insert(k.clone(), *v.choose(&mut rng).unwrap());
        }
        return strat;
    }

    pub fn get_random_start_player(player_count:usize) -> usize
    {
        let mut rng = rand::thread_rng();
        return rng.gen_range(0..player_count);
    }

    pub fn get_random_anti_card() -> Card
    {
        let mut rng = rand::thread_rng();
        let mut arr:Vec<Card> = Vec::new();
        for (k,v) in CARD_DATA.iter()
        {
            if v.anti { arr.push(*k); }
        }
        return *arr.choose(&mut rng).unwrap();
    }

    pub fn can_make_future_play(cards:&Hand, strat:&Strat) -> bool
    {
        let has_future_card = cards.contains(&Card::Future);
        let has_stealing_cards = Helpers::has_stealing_cards(cards, strat);
        return has_future_card || has_stealing_cards;
    }

    pub fn can_make_anti_play(cards:&Hand, strat:&Strat) -> bool
    {
        let has_anti_cards = Helpers::has_anti_kitten_cards(cards);
        let has_stealing_cards = Helpers::has_stealing_cards(cards, strat);
        return has_anti_cards || has_stealing_cards;
    }

    pub fn has_stealing_cards(cards:&Hand, strat:&Strat) -> bool
    {
        return Combos::get_combo(cards, strat).is_some() || cards.contains(&Card::Favor);
    }

    pub fn has_anti_kitten_cards(cards:&Hand) -> bool
    {
        for v in cards.iter()
        {
            if CARD_DATA[v].anti { return true; }
        }
        return false;
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

    // TO DO/NOTE: maybe not the best way to check agressiveness, but it's a good fit for now
    pub fn card_is_agressive(card:Card) -> bool
    {
        return CARD_DATA[&card].anti; 
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

    pub fn extract_inside_parentheses(list:Vec<String>) -> Vec<String>
    {
        let mut arr:Vec<String> = Vec::new();
        for v in list.iter()
        {
            let start_bytes = v.find("(").unwrap_or(0)+1;
            let end_bytes = v.find(")").unwrap_or(v.len());
            let result = &v[start_bytes..end_bytes];
            arr.push(result.to_owned());
        }
        return arr;
    }
}