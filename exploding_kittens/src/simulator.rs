use enum_iterator::all;

use crate::{config::{SimConfig, StratListStruct}, game::Game, results::SimResults, strats::{StratCombo, StratNope, StratPlay, Strat, StratKitten, StratVictim}};

pub struct Simulator {}

impl Simulator
{
    pub fn setup() -> SimResults
    {
        // TO DO: can we make a generic function for this instead of copy-pasting?
        let play = all::<StratPlay>().collect::<Vec<_>>();
        let nope = all::<StratNope>().collect::<Vec<_>>();
        let combo = all::<StratCombo>().collect::<Vec<_>>();
        let kitten = all::<StratKitten>().collect::<Vec<_>>();
        let victim = all::<StratVictim>().collect::<Vec<_>>();

        SimResults {
            wins_per_player: Vec::new(),
            options: StratListStruct { play, nope, combo, kitten, victim },
            strats: StratListStruct::new()
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

        res.strats.play.push(strat.play);
        res.strats.nope.push(strat.nope);
        res.strats.combo.push(strat.combo);
        res.strats.kitten.push(strat.kitten);
        res.strats.victim.push(strat.victim);
    }
}