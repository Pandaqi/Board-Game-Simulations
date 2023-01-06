use challenger::StratChall;
use config::{SimConfig, StratListStruct};
use guesser::{StratGuess, StratInitial, StratBluff};
use enum_iterator::{all};

mod simulator;
use crate::simulator::Simulator;

mod results;
use crate::results::Results;

mod game;

mod helpers;
mod challenger;
mod guesser;

mod config;

fn main() {

    // NOTE: slightly ugly way to get a list of all the options within each enum
    // How would I clean this up??
    let bluff_strats = all::<StratBluff>().collect::<Vec<_>>();
    let init_strats = all::<StratInitial>().collect::<Vec<_>>();
    let guess_strats = all::<StratGuess>().collect::<Vec<_>>();
    let prev_strats = vec![0.0, 0.25, 0.5, 0.75, 1.0];
    let next_strats = vec![0.0, 0.25, 0.5, 0.75, 1.0];
    let chall_strats = vec![-4, -3, -2, -1, 0, 1, 2, 3, 4];

    let sim_config = SimConfig {
        file_prefix: "new_system".to_string(),
        num_iterations: 1,
        player_count: 4,
        randomize_player_count: true,
        continue_until_winner: false,
        use_no_perudo_round: false,
        use_palafico_guessing: false,
        strats: StratListStruct {
            bluff: bluff_strats,
            init: init_strats,
            guess: guess_strats,
            prev_player: prev_strats,
            next_player: next_strats,
            chall_offset: chall_strats
        }
    };

    let sim_results = Simulator::simulate(&sim_config);
    Results::display(&sim_config, sim_results);
}