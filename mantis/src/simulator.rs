use std::collections::HashMap;

use crate::{results::SimResults, game::{Game, State}, config::SimConfig, strats::IdeaList};

pub struct Simulator {}

impl Simulator
{
    pub fn setup_results(cfg:&SimConfig) -> SimResults
    {
        let mut strats:IdeaList = HashMap::new();
        for k in cfg.options.keys()
        {
            strats.insert(k.clone(), Vec::new());
        }

        SimResults {
            wins_per_player: Vec::new(),
            options: cfg.options.clone(), // TO DO: prevent cloning options here; just give cfg to Results? Recalculate then?
            strats
        }
    }

    pub fn simulate() -> SimResults
    {    
        let mut cfg = SimConfig::new();
        cfg.cards = SimConfig::generate_all_cards();
        cfg.options = SimConfig::generate_all_strats(&cfg);

        let mut sim_results = Simulator::setup_results(&cfg);

        let num_sims = cfg.num_iterations;
        let print_interval = cfg.print_interval;

        for n in 0..num_sims
        {
            if n % print_interval == 0 { println!("Playing game {}", n); }
            Game::play(&mut cfg, &mut sim_results);
        }

        return sim_results;
    }

    pub fn save_results(res:&mut SimResults, state:State)
    {
        let winning_player = state.winner;
        res.wins_per_player.push(winning_player);

        let winning_strategy = state.strategies[winning_player].clone();
        for (k,v) in winning_strategy.iter()
        {
            let key = k.clone();
            let strat = v.clone();
            res.strats.get_mut(&key).unwrap().push(strat);
        }
    }
}