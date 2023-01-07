use std::collections::HashMap;

use crate::{config::{SimConfig, StratListStruct}, game::Game, results::SimResults};

pub struct Simulator {}

impl Simulator
{
    pub fn setup(cfg:&SimConfig) -> SimResults
    {
        SimResults {
            wins_per_player: Vec::new(),
            strats: StratListStruct {
                bluff: Vec::new(),
                init: Vec::new(),
                guess: Vec::new(),
                next_player: Vec::new(),
                prev_player: Vec::new(),
                chall_offset: Vec::new()
            }
        }
    }

    pub fn simulate(sim_config:&SimConfig) -> SimResults
    {    
        let mut sim_results = Simulator::setup(sim_config);

        for n in 0..sim_config.num_iterations
        {
            if n % sim_config.print_interval == 0 { println!("Playing game {}", n); }
            Game::play(sim_config, &mut sim_results);
        }

        return sim_results;
    }
}