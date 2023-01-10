use rand::Rng;

use crate::strats::{StratNope, Card, Hand, Strat, Strategy};

pub struct Nope {}

impl Nope
{
    pub fn request_based_on_strategy(strat:Strategy) -> bool
    {
        let mut prob:f64 = 0.0;
        let mut rng = rand::thread_rng();
        match strat
        {
            Strategy::Nope(StratNope::Random) => { prob = 0.1; }
            Strategy::Nope(StratNope::Never) => { prob = 0.0; }
            Strategy::Nope(StratNope::Rarely) => { prob = 0.25; }
            Strategy::Nope(StratNope::Sometimes) => { prob = 0.5; }
            Strategy::Nope(StratNope::Often) => { prob = 0.75; }
            Strategy::Nope(StratNope::Always) => { prob = 1.0; }
            _ => { }
        }
        return rng.gen::<f64>() <= prob;
    }

    pub fn opponent_will_nope(num:usize, card:Card, hands:&mut Vec<Hand>, strat:&Strat, direct_attack:bool) -> bool
    {
        if !hands[num].contains(&Card::Nope) { return false; }

        let mut will_nope:bool = false;
        let mut rng = rand::thread_rng();
        let nope_strat = *strat.get("nope").unwrap();

        match nope_strat
        {
            Strategy::Nope(StratNope::Random) => { will_nope = rng.gen::<f64>() <= 0.1; }
            Strategy::Nope(StratNope::Never) => { will_nope = false; }
            Strategy::Nope(StratNope::Rarely) => { will_nope = rng.gen::<f64>() <= 0.25; }
            Strategy::Nope(StratNope::Sometimes) => { will_nope = rng.gen::<f64>() <= 0.5; }
            Strategy::Nope(StratNope::Often) => { will_nope = rng.gen::<f64>() <= 0.75; }
            Strategy::Nope(StratNope::Always) => { will_nope = true; }
            Strategy::Nope(StratNope::OnlyIfSafe) => {
                will_nope = hands[num].len() >= 5;
            }
            Strategy::Nope(StratNope::OnlyDefuseless) => {
                will_nope = !hands[num].contains(&Card::Defuse);
            }
            Strategy::Nope(StratNope::Direct) => {
                will_nope = direct_attack;
            }
            Strategy::Nope(StratNope::DirectUnsafe) => {
                will_nope = direct_attack && !hands[num].contains(&Card::Defuse);
            }
            Strategy::Nope(StratNope::Wait) => {
                will_nope = hands.len() <= 2;
            }
            Strategy::Nope(StratNope::DeNope) => { 
                will_nope = card == Card::Nope;
            }
            Strategy::Nope(StratNope::DeNopeDirect) => {
                will_nope = direct_attack && card == Card::Nope;
            }
            _ => {}
        }

        return will_nope;
    }
}