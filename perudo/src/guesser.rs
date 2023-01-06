use std::fmt;
use enum_iterator::{Sequence};
use rand::{seq::SliceRandom, Rng};

use crate::{helpers::{Helpers, Guess, Strat}, config::SimConfig};

pub struct Guesser {}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratInitial {
    Random = 0,
    HighestValue,
    MediumValue,
    LowestValue,
    HighestNum,
    MediumNum,
    LowestNum,
    ExpectedValue
}

impl fmt::Display for StratInitial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratGuess {
    Random = 0,
    OneUpValue,
    OneUpNumber,
    TwoUpNumber,
    ThreeUpNumber,
    SmartStep,
    KnownValues,
    ExpectedValue
}

impl fmt::Display for StratGuess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

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

impl Guesser
{
    pub fn execute(cfg:&SimConfig, prev_guess:Guess, strats:&Strat, my_dice:&Vec<usize>, all_dice:&Vec<Vec<usize>>, no_perudo_round:bool) -> Guess
    {
        let mut rng = rand::thread_rng();
        let mut new_guess:Guess = (prev_guess.0, prev_guess.1);
        let total_num_dice = all_dice.iter().map(Vec::len).sum();

        let all_guesses = Helpers::get_all_valid_guesses(prev_guess, total_num_dice);
        println!("{:#?}", all_guesses);

        if Helpers::is_initial_guess(prev_guess)
        {
            match strats.init
            {
                StratInitial::Random => {
                    let val = my_dice.choose(&mut rng).unwrap();
                    let avg_num = ((total_num_dice as f64) / 3.0).round() as usize;
                    let mut num = avg_num;
                    if num >= 2 { 
                        num -= 2;
                        num += rng.gen_range(0..5); 
                    }
                    new_guess = (num, *val);
                }

                StratInitial::HighestValue => {
                    let val = Helpers::get_highest_val(&my_dice);
                    let num = Helpers::count_dice_with_val(&my_dice, val);
                    new_guess = (num, val);
                }

                StratInitial::MediumValue => {
                    let val = Helpers::get_mid_val(&my_dice);
                    let num = Helpers::count_dice_with_val(&my_dice, val);
                    new_guess = (num, val);
                }

                StratInitial::LowestValue => {
                    let val = Helpers::get_lowest_val(&my_dice);
                    let num = Helpers::count_dice_with_val(&my_dice, val);
                    new_guess = (num, val);
                }

                StratInitial::HighestNum => {
                    let val = Helpers::get_most_frequent_val(&my_dice);
                    let num = Helpers::count_dice_with_val(&my_dice, val);
                    new_guess = (num, val);
                }

                StratInitial::MediumNum => {
                    let val = Helpers::get_mid_frequent_val(&my_dice);
                    let num = Helpers::count_dice_with_val(&my_dice, val);
                    new_guess = (num, val);
                }

                StratInitial::LowestNum => {
                    let val = Helpers::get_least_frequent_val(&my_dice);
                    let num = Helpers::count_dice_with_val(&my_dice, val);
                    new_guess = (num, val);
                }

                StratInitial::ExpectedValue => {
                    let val = Helpers::get_most_frequent_val(&my_dice);
                    let exp_val = Helpers::calculate_expected_value(&my_dice, total_num_dice, val);
                    new_guess = (exp_val, val);
                }
            }
        }
        else
        {

            match strats.guess
            {
                StratGuess::Random => {
                    let mut num = prev_guess.0;
                    let mut val = prev_guess.1;
                    if rng.gen::<f64>() <= 0.5
                    {
                        num += rng.gen_range(1..4);
                    } else {
                        val += 1;
                    }
                    
                    new_guess = (num, val)
                }

                StratGuess::OneUpValue => {
                    new_guess = (prev_guess.0, prev_guess.1 + 1);
                }

                StratGuess::OneUpNumber => {
                    new_guess = (prev_guess.0 + 1, prev_guess.1);
                }

                StratGuess::TwoUpNumber => {
                    new_guess = (prev_guess.0 + 2, prev_guess.1);
                }

                StratGuess::ThreeUpNumber => {
                    new_guess = (prev_guess.0 + 3, prev_guess.1);
                }

                StratGuess::SmartStep => {
                    let mut num = prev_guess.0 + 3;
                    if !my_dice.contains(&prev_guess.1)
                    {
                        num = prev_guess.0 + 1;
                    }

                    new_guess = (num, prev_guess.1);
                }

                StratGuess::KnownValues => {
                    if my_dice.contains(&prev_guess.1) {
                        let num = prev_guess.0 + Helpers::count_dice_with_val(&my_dice, prev_guess.1);
                        new_guess = (num, prev_guess.1);
                    } else {
                        let val = Helpers::get_closest_val_ascending(&my_dice, prev_guess.1);
                        new_guess = (prev_guess.0, val);
                    }
                }

                StratGuess::ExpectedValue => {
                    let mut num = Helpers::calculate_expected_value(&my_dice, total_num_dice, prev_guess.1);
                    let mut val = prev_guess.1;
                    
                    // if this is an invalid number, raise the VALUE and calculate that expected value
                    if num <= prev_guess.0 && val < 6 {
                        val += 1;
                        num = Helpers::calculate_expected_value(&my_dice, total_num_dice, val);
                    }
                    
                    new_guess = (num, val);
                }
            }
        }

        if no_perudo_round && cfg.use_palafico_guessing
        {
            new_guess = (new_guess.0, prev_guess.1);
        }

        // cap the guess at literally impossible numbers
        new_guess = (Helpers::cap_guess_num(total_num_dice, new_guess.0), Helpers::cap_guess_val(new_guess.1));
        
        // if we end up with a guess that is not higher than the previous one, raise the number by 1 as a failsafe
        let valid_guess = (new_guess.0 > prev_guess.0) || (new_guess.1 > prev_guess.1);
        if !valid_guess { new_guess = (prev_guess.0 + 1, prev_guess.1); }

        return new_guess;
    }
}