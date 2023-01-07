use challenger::StratChall;
use config::{SimConfig, StratListStruct};
use guesser::{StratGuess, StratInitial, StratBluff};
use enum_iterator::{all};
use helpers::Strat;

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
    let chall_strats = vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5];

    let sim_config = SimConfig {
        file_prefix: "unique_name_here".to_string(),
        num_iterations: 100000,
        print_interval: 10000,
        player_count: 4,
        randomize_player_count: true,
        continue_until_winner: false,
        use_no_perudo_round: true,
        use_palafico_guessing: false,
        fixed_strat: Strat {
            bluff: StratBluff::Never, 
            init: StratInitial::Low,
            guess: StratGuess::Smallest,
            prev_player: 1.0,
            next_player: 0.0,
            chall_offset: 0
        },
        strats: StratListStruct {
            bluff: bluff_strats,
            init: init_strats,
            guess: guess_strats,
            prev_player: prev_strats,
            next_player: next_strats,
            chall_offset: chall_strats
        },
        player_data_update_increment: 0.33, // TO DO: might make this its own strategy as well??
        create_images: true,
        print_gameplay: false,
        track_per_player: false,
    };

    let sim_results = Simulator::simulate(&sim_config);
    Results::display(&sim_config, sim_results);
}