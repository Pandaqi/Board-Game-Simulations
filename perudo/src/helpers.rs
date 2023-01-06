use std::collections::HashMap;

use rand::{Rng, seq::SliceRandom};

use crate::{guesser::{StratInitial, StratGuess, StratBluff}, challenger::StratChall, config::SimConfig};

// First = number of dice, Second = value
pub type Guess = (usize, usize);

// The strategy a specific player is using
pub struct Strat {
    pub bluff: StratBluff,
    pub init: StratInitial,
    pub guess: StratGuess,
    pub prev_player: f64,
    pub next_player: f64,
    pub chall_offset: i32
}

pub struct Helpers {}

impl Helpers 
{
    pub fn get_all_valid_guesses(cur_guess:Guess, num_dice:usize) -> Vec<Guess>
    {
        let mut arr:Vec<Guess> = Vec::new();

        // if we increase the VALUE, we can turn the other number into anything
        let start_bound = cur_guess.1 + 1;
        if start_bound <= 6
        {
            for i in start_bound..=6
            {
                for j in 1..=num_dice
                {
                    arr.push((j, i))
                }
            } 
        }
        
        // if we keep VALUE the same, the other number must be higher than what it is now
        let start_bound_num = cur_guess.0 + 1;
        for i in start_bound_num..=num_dice
        {
            arr.push((i, cur_guess.1));
        }

        return arr;
    }

    pub fn pick_random_strategy(cfg:&SimConfig) -> Strat
    {
        let mut rng = rand::thread_rng();
        let strat = Strat {
            bluff: *cfg.strats.bluff.choose(&mut rng).unwrap(),
            init: *cfg.strats.init.choose(&mut rng).unwrap(),
            guess: *cfg.strats.guess.choose(&mut rng).unwrap(),
            prev_player: *cfg.strats.prev_player.choose(&mut rng).unwrap(),
            next_player: *cfg.strats.next_player.choose(&mut rng).unwrap(),
            chall_offset: *cfg.strats.chall_offset.choose(&mut rng).unwrap(),
        };
        return strat;
    }

    pub fn get_random_die_value() -> usize
    {
        let mut rng = rand::thread_rng();
        return rng.gen_range(1..7) as usize;
    }
    
    pub fn get_start_player(count:usize) -> usize
    {
        let mut rng = rand::thread_rng();
        return rng.gen_range(0..count) as usize;
    }
    
    pub fn is_initial_guess(guess:Guess) -> bool
    {
        return guess.0 == 0 && guess.1 == 0;
    }

    pub fn cap_guess_num(num_dice:usize, num:usize) -> usize
    {
        if num >= num_dice { return num_dice; }
        if num <= 1 { return 1; }
        return num;
    }

    pub fn cap_guess_val(val:usize) -> usize
    {
        if val >= 6 { return 6; }
        if val <= 1 { return 1; }
        return val;
    }

    pub fn get_highest_val(dice:&Vec<usize>) -> usize
    {
        return *dice.iter().max().unwrap();
    }

    pub fn get_lowest_val(dice:&Vec<usize>) -> usize
    {
        return *dice.iter().min().unwrap();
    }

    pub fn get_mid_val(dice:&Vec<usize>) -> usize
    {
        let max = Helpers::get_highest_val(dice);
        let min = Helpers::get_lowest_val(dice);
        for v in dice.iter()
        {
            if *v == min || *v == max { continue; }
            return *v;
        }
        return max;
    }

    pub fn get_most_frequent_val(dice:&Vec<usize>) -> usize
    {
        let vec = Helpers::sort_by_frequency(dice);
        return vec[vec.len() - 1];
    }

    pub fn get_mid_frequent_val(dice:&Vec<usize>) -> usize
    {
        let vec = Helpers::sort_by_frequency(dice);
        let avg_idx = (0.5 * (vec.len() as f64)).floor() as usize;
        return vec[avg_idx];
    }

    pub fn get_least_frequent_val(dice:&Vec<usize>) -> usize
    {
        let vec = Helpers::sort_by_frequency(dice);
        return vec[0];
    }

    pub fn get_closest_val_ascending(dice:&Vec<usize>, val:usize) -> usize
    {
        let mut closest_dist:i32 = 1000;
        let mut closest_val:usize = 0;
        for v in dice.iter()
        {
            let dist:i32 = (*v as i32) - (val as i32);
            if dist < 0 { continue; }
            if dist >= closest_dist { continue; }

            closest_dist = dist;
            closest_val = *v;
        }
        return closest_val;
    }

    pub fn calculate_expected_value(dice:&Vec<usize>, num_dice:usize, val:usize) -> usize
    {
        let unknown_dice = num_dice - dice.len();
        let my_input = Helpers::count_dice_with_val(dice, val);
        let prob = 2.0/6.0; // either the same number or a perudo/ace 
        let their_input = (unknown_dice as f64) * prob;
        return (my_input as f64 + their_input).round() as usize;
    }

    // This sorts ASCENDING
    pub fn sort_by_frequency(dice:&Vec<usize>) -> Vec<usize>
    {
        let mut m: HashMap<usize, usize> = HashMap::new();
        for v in dice {
            *m.entry(*v).or_default() += 1;
        }

        let mut arr:Vec<usize> = m.clone().into_keys().collect();
        arr.sort_by(|a, b| m[a].partial_cmp(&m[b]).unwrap());

        return arr.clone();
    }

    pub fn count_dice_with_val(dice:&Vec<usize>, val:usize) -> usize
    {
        return dice.iter().filter(|&n| *n == val).count()
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

    /*
    pub fn to_string_list<T>(map:&HashMap<T, i32>) -> Vec<String> where T: std::fmt::Display
    {
        let mut arr:Vec<String> = Vec::new();
        for (k,_v) in map
        {
            arr.push(k.to_string());
        }
        return arr;
    }
    */

    pub fn to_flat_string_list<T>(map:&HashMap<T, i32>) -> Vec<String> where T: std::fmt::Display
    {
        let mut arr:Vec<String> = Vec::new();
        for (k,v) in map
        {
            for _i in 0..*v
            {
                arr.push(k.to_string());
            }
        }
        return arr;
    }
}