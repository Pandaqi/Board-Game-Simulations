use std::fmt;
use enum_iterator::{Sequence};

use crate::{helpers::{Helpers}, game::{GuessParams, PlayerData}, config::SimConfig};

pub struct Challenger {}

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
    pub fn wants_to_challenge(params:&GuessParams) -> bool
    {
        if Helpers::is_initial_guess(params.prev_guess) { return false; }

        // if we KNOW this is (almost certainly) true
        let our_num_of_type = Helpers::count_dice_with_val(&params.my_dice, params.prev_guess.1, params.no_perudo_round);
        let declared_num = params.prev_guess.0;
        if declared_num <= our_num_of_type || declared_num <= 2 { return false; }

        let total_num_dice:usize = params.all_dice.iter().map(Vec::len).sum();
        let palafico = params.no_perudo_round && params.use_palafico_guessing;
        let all_guesses = Helpers::get_all_valid_guesses(&params.guess_history, total_num_dice, palafico);
        let remaining_guesses = all_guesses.len();

        // hard override to ensure we challenge when that's literally the only thing to do
        let no_valid_guess = remaining_guesses <= 1;
        if no_valid_guess { return true; }
        
        // if the number sounds ridiculous to us, challenge
        let expected_value = Helpers::calculate_expected_value(&params.my_dice, total_num_dice, params.prev_guess.1, params.no_perudo_round) as i32;
        let guessed_value = params.prev_guess.0 as i32;
        let mut challenge = guessed_value >= (expected_value + params.strat.chall_offset);

        // however, overrule this if we think the last player bluffed, or the next player won't challenge anyway
        if Helpers::will_prev_player_bluff(&params.player_data, &params.strat) {
            challenge = true;
        } else if Helpers::will_next_player_challenge(&params.player_data, &params.strat, remaining_guesses) {
            challenge = false;
        }

        return challenge;
    }

    pub fn execute(cfg:&SimConfig, params:&GuessParams, all_player_data:&mut Vec<PlayerData>, cur_player:usize, prev_player:usize, num_dice:&mut Vec<usize>) -> bool
    {
        // let's count the numbers and see if we're correct
        let mut total_of_value = 0;
        let wanted_number_of_given_value = params.prev_guess.0;
        let wanted_value = params.prev_guess.1;
        for v1 in params.all_dice.iter()
        {
            total_of_value += Helpers::count_dice_with_val(v1, wanted_value, params.no_perudo_round);
        }

        let correct_challenge:bool = wanted_number_of_given_value > total_of_value;
        let data_inc = cfg.player_data_update_increment;
        if correct_challenge
        {
            Helpers::print(cfg, "CHALLENGE; WON");

            // we won, so the previous player bluffed, so raise that chance
            all_player_data[cur_player].prev = (all_player_data[cur_player].prev + data_inc).clamp(0.0, 1.0);

            num_dice[prev_player] -= 1;

            // the player who lost now knows the next player is likely to challenge, raise the chance
            all_player_data[prev_player].next = (all_player_data[prev_player].next + data_inc).clamp(0.0, 1.0);

            return true;
        }
        else
        {
            Helpers::print(cfg, "CHALLENGE; LOST");

            num_dice[cur_player] -= 1;

            // we lost, so we know the previous player is unlikely to bluff
            all_player_data[cur_player].prev = (all_player_data[cur_player].prev - data_inc).clamp(0.0, 1.0);

            // the player who won now knows we are unlikely to challenge again
            all_player_data[prev_player].next = (all_player_data[prev_player].next - data_inc).clamp(0.0, 1.0);

            return false;
        }
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