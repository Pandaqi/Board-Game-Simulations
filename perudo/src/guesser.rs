use std::fmt;
use enum_iterator::{Sequence};
use rand::{seq::SliceRandom, Rng};

use crate::{helpers::{Helpers, Guess, Strat}, config::SimConfig, game::GuessParams};

pub struct Guesser {}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratBluff {
    Always = 0,
    High,
    Medium,
    Low,
    Never,
    LowToHigh,
    HighToLow
}

impl fmt::Display for StratBluff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratInitial {
    Random = 0,
    Low,
    High
}

impl fmt::Display for StratInitial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratGuess {
    Random = 0,
    Smallest,
    Medium,
    Biggest,
    Known,
    Expected,
    KnownHigh,
    LowerNum,
    HigherNum
}

impl fmt::Display for StratGuess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Guesser
{
    pub fn execute(params:&GuessParams) -> Guess
    {
        let mut new_guess:Guess = params.prev_guess.clone();

        if Helpers::is_initial_guess(new_guess) {
            new_guess = Guesser::create_initial_guess(params);
        } else {
            new_guess = Guesser::create_guess(params)
        }
        return new_guess;
    }

    fn create_initial_guess(params:&GuessParams) -> Guess
    {
        let mut rng = rand::thread_rng();
        let mut new_guess:Guess = params.prev_guess;
        let total_num_dice:usize = params.all_dice.iter().map(Vec::len).sum();
        let player_count = params.all_dice.len();

        // initial guess may never be about aces; unless it's a palafico round
        let mut dice_without_aces = Vec::new();
        let is_palafico = params.use_palafico_guessing && params.no_perudo_round;
        for v in params.my_dice.iter()
        {
            if *v == 1 && !is_palafico { continue; }
            dice_without_aces.push(*v);
        }

        // could happen that we have nothing to guess then
        // which means we just pick a random number
        if dice_without_aces.len() <= 0 { dice_without_aces.push(rng.gen_range(2..7)); }

        match params.strat.init
        {
            StratInitial::Random => {
                let val = *dice_without_aces.choose(&mut rng).unwrap();
                let mut avg_num = ((total_num_dice as f64) / 3.0).round() as usize;
                if avg_num < 2 { avg_num = 2; }
                let num = avg_num + rng.gen_range(0..5) - 2;
                new_guess = (num, val);
            }

            StratInitial::Low => {
                let val = *dice_without_aces.choose(&mut rng).unwrap();
                let num = rng.gen_range(1..3);
                new_guess = (num, val);
            }

            StratInitial::High => {
                let val = *dice_without_aces.choose(&mut rng).unwrap();
                let low_bound = player_count - 1;
                let range = low_bound..(2*low_bound);
                if !range.is_empty()
                {
                    let num = rng.gen_range(range);
                    new_guess = (num, val);
                }
                
            }        
        }

        // If we're bluffing, 
        // 1) we either pick a dice value we don't even have
        // 2) or we go way higher than expected value on our number
        if Helpers::should_bluff(&params.strat, &params.my_dice)
        {
            if rng.gen::<f64>() <= 0.7
            {
                let val = Helpers::pick_non_existent_dice_value_no_aces(&params.my_dice);
                new_guess = (new_guess.0, val);
            } 
            else 
            {
                let mut num = Helpers::count_dice_with_val(&params.my_dice, new_guess.1, params.no_perudo_round);
                num += rng.gen_range(1..=3);
                new_guess = (num, new_guess.1);
            }
        }

        // clamping for safety
        new_guess = (new_guess.0.clamp(1, total_num_dice), new_guess.1);

        return new_guess;
    }

    pub fn create_guess(params:&GuessParams) -> Guess
    {
        let total_num_dice = params.all_dice.iter().map(Vec::len).sum();
        let palafico = params.no_perudo_round && params.use_palafico_guessing;
        let all_guesses = Helpers::get_all_valid_guesses(&params.guess_history, total_num_dice, palafico);

        if all_guesses.len() <= 1 { return (0,0); }

        let mut rng = rand::thread_rng();
        let mut new_guess = params.prev_guess;
        let mut guess_idx = rng.gen_range(0..all_guesses.len());

        let bluff = Helpers::should_bluff(&params.strat, &params.my_dice);
        let mut bluff_raises_number:bool = false;
            
        match params.strat.guess
        {
            StratGuess::Random => {} // random is default anyway
            StratGuess::Smallest => {
                bluff_raises_number = true;
                let first_third = (0.33 * (all_guesses.len() as f64)).ceil() as usize;
                let range = 0..first_third;

                if !range.is_empty()
                { 
                    guess_idx = rng.gen_range(range);
                }
            }

            StratGuess::Medium => {
                bluff_raises_number = true;

                let center:i32 = (0.5 * (all_guesses.len() as f64)).round() as i32;
                let max = (all_guesses.len() - 1) as i32;
                let bottom = (center-4).clamp(0, max) as usize;
                let top = (center+4).clamp(0, max) as usize;
                let range = bottom..=top;
                if !range.is_empty()
                {
                    guess_idx = rng.gen_range(range);
                }
            }

            StratGuess::Biggest => {
                let latter_third = (0.66 * (all_guesses.len() as f64)).floor() as usize;
                let range = latter_third..all_guesses.len();
                if !range.is_empty() 
                {
                    guess_idx = rng.gen_range(range);
                }
            }

            StratGuess::Known => {
                let mut options = Vec::new();
                let cur_num = params.prev_guess.0;
                for i in 0..all_guesses.len()
                {
                    let has_type:bool = params.my_dice.contains(&all_guesses[i].1);
                    // reverses it when bluffing, so you pick something you DON'T have
                    if bluff == has_type { continue; } 
                    if all_guesses[i].0 > (cur_num + 4) { continue; }
                    options.push(i);
                }
                
                if options.len() > 0 { guess_idx = *options.choose(&mut rng).unwrap(); }
            }

            // pick an allowed guess where we're close to the expected value
            StratGuess::Expected => {
                let mut options = Vec::new();
                for i in 0..all_guesses.len()
                {
                    let num = &all_guesses[i].0;
                    let val = &all_guesses[i].1;
                    let mut exp_val = Helpers::calculate_expected_value(&params.my_dice, total_num_dice, *val, params.no_perudo_round);

                    if bluff { exp_val += rng.gen_range(1..=3); }

                    let dist = (*num as i32 - exp_val as i32).abs();
                    if dist > 2 { continue; }
                    options.push(i);
                }

                if options.len() > 0 { guess_idx = *options.choose(&mut rng).unwrap(); }
            }

            // keep number the same + 1, but switch to a known value (if needed)
            StratGuess::KnownHigh =>
            {
                let mut options = Vec::new();
                let cur_num = params.prev_guess.0;
                for i in 0..all_guesses.len()
                {
                    let has_type:bool = params.my_dice.contains(&all_guesses[i].1);
                    if bluff == has_type { continue; } 
                    if all_guesses[i].0 < (cur_num + 1) { continue; }
                    options.push(i);
                }
                
                if options.len() > 0 { guess_idx = *options.choose(&mut rng).unwrap(); }
            }

            // pick whatever allows you to lower the NUMBER
            StratGuess::LowerNum =>
            {
                let mut options = Vec::new();
                let cur_num = params.prev_guess.0;
                for i in 0..all_guesses.len()
                {
                    let has_type:bool = params.my_dice.contains(&all_guesses[i].1);

                    // slightly different: only if we bluff, do we want to purposely skip values we have
                    if bluff && has_type { continue; } 
                    if all_guesses[i].0 >= cur_num { continue; }
                    options.push(i);
                }

                if options.len() > 0 { guess_idx = *options.choose(&mut rng).unwrap(); }
            }

            // pick whatever allows you to raise the NUMBER
            // (by at most 4, otherwise we can get ridiculous numbers)
            StratGuess::HigherNum =>
            {
                bluff_raises_number = true;

                let mut options = Vec::new();
                let cur_num = params.prev_guess.0;
                for i in 0..all_guesses.len()
                {
                    if all_guesses[i].0 <= cur_num { continue; }
                    if all_guesses[i].0 >= (cur_num + 5) { continue; }
                    options.push(i);
                }
                
                if options.len() > 0 { guess_idx = *options.choose(&mut rng).unwrap(); }
            }
        }

        // If the next player is unlikely to challenge (according to our rough prediction)
        // Or we're bluffing and want to raise
        // Raise the number a bit further (as long as we stay on the same value)
        let remaining_guesses = all_guesses.len() - (guess_idx + 1);
        if !Helpers::will_next_player_challenge(&params.player_data, &params.strat, remaining_guesses) 
            || (bluff_raises_number && bluff)
        {
            let cur_val = all_guesses[guess_idx].1;
            for _i in 0..3
            {
                if all_guesses[guess_idx].1 != cur_val { break; }
                guess_idx += 1;
                if guess_idx >= all_guesses.len() { break; }
            }
        }

        guess_idx = guess_idx.clamp(0, all_guesses.len()-1);
        new_guess = all_guesses[guess_idx];

        return new_guess;
    }
}