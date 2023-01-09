use rand::Rng;

use crate::strats::{StratNope, Card, Hand, Strat};

pub struct Nope {}

impl Nope
{
    pub fn request_based_on_strategy(strat:StratNope) -> bool
    {
        let mut prob:f64 = 0.0;
        let mut rng = rand::thread_rng();
        match strat
        {
            StratNope::Random => { prob = 0.1; }
            StratNope::Never => { prob = 0.0; }
            StratNope::Rarely => { prob = 0.25; }
            StratNope::Sometimes => { prob = 0.5; }
            StratNope::Often => { prob = 0.75; }
            StratNope::Always => { prob = 1.0; }
            _ => { }
        }
        return rng.gen::<f64>() <= prob;
    }

    pub fn opponent_will_nope(num:usize, card:Card, hands:&mut Vec<Hand>, strat:&Strat, direct_attack:bool) -> bool
    {
        if !hands[num].contains(&Card::Nope) { return false; }

        let will_nope:bool;
        let mut rng = rand::thread_rng();
        match strat.nope
        {
            StratNope::Random => { will_nope = rng.gen::<f64>() <= 0.1; }
            StratNope::Never => { will_nope = false; }
            StratNope::Rarely => { will_nope = rng.gen::<f64>() <= 0.25; }
            StratNope::Sometimes => { will_nope = rng.gen::<f64>() <= 0.5; }
            StratNope::Often => { will_nope = rng.gen::<f64>() <= 0.75; }
            StratNope::Always => { will_nope = true; }
            StratNope::OnlyIfSafe => {
                will_nope = hands[num].len() >= 5;
            }
            StratNope::OnlyDefuseless => {
                will_nope = !hands[num].contains(&Card::Defuse);
            }
            StratNope::Direct => {
                will_nope = direct_attack;
            }
            StratNope::DirectUnsafe => {
                will_nope = direct_attack && !hands[num].contains(&Card::Defuse);
            }
            StratNope::Wait => {
                will_nope = hands.len() <= 2;
            }
            StratNope::DeNope => { 
                will_nope = card == Card::Nope;
            }
            StratNope::DeNopeDirect => {
                will_nope = direct_attack && card == Card::Nope;
            }
        }

        return will_nope;
    }
}