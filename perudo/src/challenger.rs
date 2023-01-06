use std::fmt;
use enum_iterator::{Sequence};
use rand::Rng;

use crate::helpers::{Helpers, Guess, Strat};

pub struct Challenger {}

pub enum ChallengeResult {
    None,
    Won,
    Lost
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratChall {
    Random,
    ExpectedValue,
    AboveSeven
}

impl fmt::Display for StratChall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Challenger
{
    pub fn check(prev_guess:Guess, strats:&Strat, my_dice: &Vec<usize>, all_dice:&Vec<Vec<usize>>, no_perudo_round:bool) -> ChallengeResult
    {
        if Helpers::is_initial_guess(prev_guess) { return ChallengeResult::None; }
        
        let mut challenge: bool = false;
        let mut rng = rand::thread_rng();
        let total_num_dice:usize = all_dice.iter().map(Vec::len).sum();

        // TO DO: challenge calculations based on the three properties

        if !challenge { return ChallengeResult::None; }

        // we've decided to challenge
        // let's count the numbers and see if we're correct
        let mut total_of_value = 0;
        let wanted_number_of_given_value = prev_guess.0;
        let wanted_value = prev_guess.1;
        for v1 in all_dice.iter()
        {
            for v2 in v1.iter()
            {
                let mut its_a_match:bool = v2.to_owned() == wanted_value;
                if !its_a_match && !no_perudo_round
                {
                    its_a_match = v2.to_owned() == 1;
                }

                if !its_a_match { continue; }
                total_of_value += 1;
            }
        }

        let correct_challenge:bool = wanted_number_of_given_value <= total_of_value;
        if correct_challenge { return ChallengeResult::Won; }

        return ChallengeResult::Lost;
    }
}

/* OLD CHALLENGE CODE 

match strats.2
{
    StratChall::Random => {
        challenge = rng.gen::<f64>() <= 0.33;
    }

    StratChall::ExpectedValue => {
        let num = Helpers::calculate_expected_value(my_dice, total_num_dice, prev_guess.1);
        challenge = prev_guess.0 > num;
    }

    StratChall::AboveSeven => {
        challenge = prev_guess.0 > 7
    }
}


*/