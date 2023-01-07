use std::collections::HashMap;

use rand::{Rng, seq::SliceRandom};

use crate::{guesser::{StratInitial, StratGuess, StratBluff}, challenger::StratChall, config::SimConfig, game::PlayerData};

// First = number of dice, Second = value
pub type Guess = (usize, usize);

// The strategy a specific player is using
#[derive(Copy, Clone, Debug)]
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
    pub fn print<T>(cfg:&SimConfig, txt:T) where T: std::fmt::Debug
    {
        if !cfg.print_gameplay { return; }
        println!("{:#?}", txt);
    }

    pub fn should_bluff(strat:&Strat, my_dice:&Vec<usize>) -> bool
    {
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen::<f64>();
        let num_dice = my_dice.len();

        match strat.bluff
        {
            StratBluff::Never => { return false; }
            StratBluff::Always => { return true; }
            StratBluff::High => {  return rand_num <= 0.75; }
            StratBluff::Medium => { return rand_num <= 0.5; }
            StratBluff::Low => { return rand_num <= 0.25; }
            StratBluff::LowToHigh => {
                if num_dice <= 2 { return rand_num <= 0.75; }
                return rand_num <= 0.25;
            }
            StratBluff::HighToLow => {
                if num_dice <= 2 { return rand_num <= 0.25; }
                return rand_num <= 0.75;
            }
        }
    }

    pub fn will_prev_player_bluff(data:&PlayerData, strat:&Strat) -> bool
    {
        let mut rng = rand::thread_rng();
        let prob = strat.prev_player * data.prev.clamp(0.0, 1.0);
        return rng.gen::<f64>() <= prob;
    }

    pub fn will_next_player_challenge(data:&PlayerData, strat:&Strat, remaining_guesses:usize) -> bool
    {
        if remaining_guesses <= 1 { return true; }
        
        let mut rng = rand::thread_rng();
        let prob = strat.next_player * data.next.clamp(0.0, 1.0);
        return rng.gen::<f64>() <= prob;
    }

    // NOTE: these are automatically ORDERED from safest to least safe bet
    // (just by how the loops are structured)
    pub fn get_all_valid_guesses(guess_history:&Vec<Guess>, num_dice:usize, palafico:bool) -> Vec<Guess>
    {
        let mut arr:Vec<Guess> = Vec::new();
        let cur_guess = guess_history.last().unwrap();

        // TO DO?? Put the rule about ace bids behind a cfg toggle?
        let cur_bid_is_aces = cur_guess.1 == 1;

        // consider all combinations
        // only allow those where at least ONE of the values is higher
        for i in 1..=num_dice
        {
            let start_val = if cur_bid_is_aces { 1 } else { 2 };
            for j in start_val..=6
            {
                if palafico && j != cur_guess.1 { continue; } // palafico = during a no perudo round, you're only allowed to raise NUMBER, not VALUE
                if cur_bid_is_aces && j != 1 && i < (2*cur_guess.0 + 1) { continue; } // to switch from ace bid to normal numbers, you must raise the number and add 1 (at least) 
                if i <= cur_guess.0 { continue; }
                arr.push((i, j));
            }
        }

        if !cur_bid_is_aces && cur_guess.0 >= 4
        {
            let half_prev_bid = (0.5 * (cur_guess.0 as f64)).ceil() as usize;
            let ace_bid = (half_prev_bid, 1);
            let mut already_guessed = false;
            for v in guess_history.iter()
            {
                if !(v.0 == ace_bid.0 && v.1 == ace_bid.1) { continue; }
                already_guessed = true;
                break;
            }

            if !already_guessed
            {
                arr.push(ace_bid);
            }     
        }

        return arr;
    }

    pub fn count_players_left(all_dice:&Vec<Vec<usize>>) -> usize
    {
        let mut count = 0;
        for v in all_dice.iter()
        {
            if v.len() <= 0 { continue; }
            count += 1;
        }
        return count;
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

    pub fn pick_non_existent_dice_value_no_aces(dice:&Vec<usize>) -> usize
    {
        let all_values = vec![2,3,4,5,6];
        for v in all_values.iter()
        {
            if dice.contains(v) { continue; }
            return *v;
        }
        return 0;
    }

    pub fn get_random_die_value() -> usize
    {
        let mut rng = rand::thread_rng();
        return rng.gen_range(1..7) as usize;
    }
    
    pub fn get_start_player(count:usize) -> usize
    {
        let mut rng = rand::thread_rng();
        let range = 0..count;
        if range.is_empty() { return 0; }
        return rng.gen_range(range) as usize;
    }
    
    pub fn is_initial_guess(guess:Guess) -> bool
    {
        return guess.0 == 0 && guess.1 == 0;
    }

    pub fn calculate_expected_value(dice:&Vec<usize>, num_dice:usize, val:usize, no_perudo_round:bool) -> usize
    {
        let unknown_dice = num_dice - dice.len();
        let my_input = Helpers::count_dice_with_val(dice, val, no_perudo_round);
        let mut prob = 2.0/6.0; // either the same number or a perudo/ace 
        if no_perudo_round { prob = 1.0/6.0; }

        let their_input = (unknown_dice as f64) * prob;
        let mut exp_val = (my_input as f64 + their_input).floor() as usize;
        exp_val = exp_val.clamp(1, num_dice-1);
        return exp_val;
    }

    pub fn count_dice_with_val(dice:&Vec<usize>, val:usize, no_perudo_round:bool) -> usize
    {
        let mut sum = 0;
        for v in dice.iter()
        {
            if !(*v == val || (*v == 1 && !no_perudo_round)) { continue; }
            sum += 1;
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