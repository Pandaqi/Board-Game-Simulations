mod simulator;
use crate::simulator::Simulator;

mod results;
use crate::results::Results;

mod config;

mod game;
mod test;
mod strats;
mod helpers;
mod display;

fn main() {
    let sim_results = Simulator::simulate();
    Results::display(sim_results);
}