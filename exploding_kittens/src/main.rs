mod simulator;
use config::SimConfig;

use crate::simulator::Simulator;

mod results;
use crate::results::Results;

mod config;

mod game;
mod helpers;
mod test;
mod strats;

fn main() {
    let sim_config = SimConfig::new();
    let sim_results = Simulator::simulate(&sim_config);
    Results::display(&sim_config, sim_results);
}