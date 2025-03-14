use enum_iterator::all;

use crate::{strats::Idea, game::{Card, Hand, State}};

pub struct Helpers {}

impl Helpers 
{
    pub fn create_enum_match_list(full:&Vec<Idea>, key:Idea) -> Vec<Idea>
    {
        let mut arr:Vec<Idea> = Vec::new();
        for v in full.iter()
        {
            if std::mem::discriminant(v) != std::mem::discriminant(&key) { continue; }
            arr.push(v.clone());
        }
        return arr;
    }

    pub fn hand_matches_card_back(card:&Card, hand:&Hand) -> bool
    {
        return hand.contains_key(&card.color) || hand.contains_key(&card.back1) || hand.contains_key(&card.back2);
    }

    pub fn color_matches_card_back(card:&Card, color:u8) -> bool
    {
        return card.color == color || card.back1 == color || card.back2 == color;
    }

    pub fn get_lead(score:usize, state:&State) -> i32
    {
        let mut lead = 1000;
        for (_k,v) in state.score.iter().enumerate()
        {
            let dist = (*v as i32 - score as i32).abs();
            if dist == 0 || dist >= lead { continue; }
            lead = dist;
        }
        return lead;
    }

    pub fn get_player_with_highest_score(state:&State) -> usize
    {
        let mut highest_score = 0;
        let mut player = 0;
        for (k,v) in state.score.iter().enumerate()
        {
            if *v <= highest_score { continue; }
            highest_score = *v;
            player = k;
        }
        return player;
    }

    pub fn get_player_with_lowest_score(state:&State) -> usize
    {
        let mut lowest_score = 0;
        let mut player = 0;
        for (k,v) in state.score.iter().enumerate()
        {
            if *v >= lowest_score { continue; }
            lowest_score = *v;
            player = k;
        }
        return player;
    }

    pub fn is_guaranteed_steal(card:&Card, hand:&Hand) -> bool
    {
        return Helpers::count_matching_colors(card, hand) >= 3;
    }

    pub fn score_is_guaranteed_win(state:&State) -> bool
    {
        return Helpers::is_guaranteed_win(state, state.cur_player);
    }
    
    pub fn is_guaranteed_win(state:&State, num:usize) -> bool
    {
        let top_card = state.deck.last().unwrap();
        if !Helpers::is_guaranteed_steal(top_card, &state.hands[num]) { return false; }

        let points_needed = (state.score_threshold - state.score[num]) as i32;
        let points_available = Helpers::get_smallest_matching_stack(top_card, &state.hands[num]);
        let not_enough_points = points_available < points_needed;
        if not_enough_points { return false; }
        
        return true;
    }

    pub fn get_smallest_matching_stack(card:&Card, hand:&Hand) -> i32
    {
        let mut smallest = 1000;
        for (color, amount) in hand.iter()
        {
            if !Helpers::color_matches_card_back(card, *color) { continue; }
            if *amount >= smallest { continue; }
            smallest = *amount;
        }
        return smallest as i32;
    }

    pub fn get_biggest_stack(hand:&Hand) -> i32
    {
        let mut biggest:i32 = 0;
        for (_k,v) in hand.iter()
        {
            if *v as i32 <= biggest { continue; }
            biggest = *v as i32;
        }
        return biggest;
    }

    pub fn count_stacks_bigger_than(threshold:usize, hand:&Hand) -> i32
    {
        let mut sum:i32 = 0;
        for (_k,v) in hand.iter()
        {
            if *v < threshold { continue; }
            sum += 1;
        }
        return sum;
    }

    pub fn count_stacks_smaller_than(threshold:usize, hand:&Hand) -> i32
    {
        let mut sum:i32 = 0;
        for (_k,v) in hand.iter()
        {
            if *v > threshold { continue; }
            sum += 1;
        }
        return sum;
    }

    pub fn count_matching_cards(card:&Card, hand:&Hand) -> i32
    {
        let mut sum:i32 = 0;
        for (color, amount) in hand.iter()
        {
            if !Helpers::color_matches_card_back(card, *color) { continue; }
            sum += *amount as i32;
        }
        return sum;
    }

    pub fn count_matching_colors(card:&Card, hand:&Hand) -> i32
    {
        let mut sum:i32 = 0;
        for (color, amount) in hand.iter()
        {
            if !Helpers::color_matches_card_back(card, *color) { continue; }
            sum += 1;
        }
        return sum;
    }

    pub fn get_all_possible_cards() -> Vec<Card>
    {
        return all::<Card>().collect::<Vec<_>>();
    }

    pub fn count_cards_total_all_players(hands:&Vec<Hand>) -> usize
    {
        let mut sum = 0;
        for v in hands
        {
            sum += Helpers::count_cards_total(v);
        }
        return sum;
    }

    pub fn count_cards_total(hand:&Hand) -> usize
    {
        let mut sum = 0;
        for (_k,v) in hand.iter()
        {
            sum += *v;
        }
        return sum;
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

    pub fn extract_inside_parentheses_single(txt:&String) -> String
    {
        let start_bytes = txt.find("(").unwrap_or(0)+1;
        let end_bytes = txt.find(")").unwrap_or(txt.len());
        let result = &txt[start_bytes..end_bytes];
        return result.to_owned();
    }

    pub fn extract_inside_parentheses(list:Vec<String>) -> Vec<String>
    {
        let mut arr:Vec<String> = Vec::new();
        for v in list.iter()
        {
            arr.push(Helpers::extract_inside_parentheses_single(v));
        }
        return arr;
    }
}