use std::collections::HashMap;

use enum_iterator::all;

use crate::{config::{SimConfig}, game::Game, results::SimResults, strats::{StratCombo, StratNope, StratPlay, Strat, StratKitten, StratVictim, StratList, Strategy}, helpers::Helpers};

pub struct Simulator {}

impl Simulator
{
    pub fn setup() -> SimResults
    {
        
        // this lists ALL strategies across all categories
        // then we grab slices to save them per category
        let fields_auto = all::<Strategy>().collect::<Vec<_>>();

        // TO DO: this is the ONE thing I need to manually update
        let options:StratList = HashMap::from([
            ("play".to_owned(), Helpers::create_enum_match_list(&fields_auto, Strategy::Play(StratPlay::All))),
            ("nope".to_owned(), Helpers::create_enum_match_list(&fields_auto, Strategy::Nope(StratNope::Always))),
            ("combo".to_owned(), Helpers::create_enum_match_list(&fields_auto, Strategy::Combo(StratCombo::AllCards))),
            ("kitten".to_owned(), Helpers::create_enum_match_list(&fields_auto, Strategy::Kitten(StratKitten::Bottom))),
            ("victim".to_owned(), Helpers::create_enum_match_list(&fields_auto, Strategy::Victim(StratVictim::Random)))
        ]);

        let mut strats:StratList = HashMap::new();
        for k in options.keys()
        {
            strats.insert(k.clone(), Vec::new());
        }

        SimResults {
            wins_per_player: Vec::new(),
            options,
            strats
        }
    }

    pub fn simulate(cfg:&SimConfig) -> SimResults
    {    
        let mut sim_results = Simulator::setup();
        let mut game:Game = Game::new();
        game.setup();

        for n in 0..cfg.num_iterations
        {
            if n % cfg.print_interval == 0 { println!("Playing game {}", n); }
            game.play(cfg, &mut sim_results);
        }

        return sim_results;
    }

    pub fn save_results(res:&mut SimResults, num: usize, strat:Strat)
    {
        res.wins_per_player.push(num);

        for (k,v) in strat.iter()
        {
            let key = k.clone();
            let strat = v.clone();
            res.strats.get_mut(&key).unwrap().push(strat);
        }
    }
}