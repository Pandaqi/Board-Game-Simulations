use rand::Rng;

use crate::{strats::{StratNope, Card, Hand, Strat, Strategy, StratNopeCustom}, helpers::Helpers};

pub struct Nope {}

impl Nope
{
    pub fn play_based_on_strategy(strat:&Strat) -> bool
    {
        let nope_strat = *strat.get("nope").unwrap();
        if let Strategy::Nope(tp) = nope_strat
        {
            let prob = ((tp as usize) as f64) / 15.0;
            let mut rng = rand::thread_rng();
            return rng.gen::<f64>() <= prob;
        }
        return false;
    }

    pub fn defend_based_on_strategy(strat:&Strat) -> bool
    {
        let nope_defend = *strat.get("nope_defend").unwrap();
        if let Strategy::NopeDefend(tp) = nope_defend
        {
            let prob = ((tp as usize) as f64) / 15.0;
            let mut rng = rand::thread_rng();
            return rng.gen::<f64>() <= prob;
        }
        return false;
    }

    pub fn opponent_will_nope(num:usize, card:Card, hands:&mut Vec<Hand>, strat:&Strat, direct_attack:bool) -> bool
    {
        if !hands[num].contains(&Card::Nope) { return false; }

        let wants_to_nope = Nope::play_based_on_strategy(strat) || (direct_attack && Nope::defend_based_on_strategy(strat));
        let nope_custom_strat = *strat.get("nope_custom").unwrap();
        if nope_custom_strat == Strategy::NopeCustom(StratNopeCustom::Pass) { return wants_to_nope; }

        let mut override_nope:bool = false;
        match nope_custom_strat
        {
            Strategy::NopeCustom(StratNopeCustom::IfSafe) => { override_nope = hands[num].contains(&Card::Defuse); }
            Strategy::NopeCustom(StratNopeCustom::IfUnsafe) => { override_nope = !hands[num].contains(&Card::Defuse); }
            Strategy::NopeCustom(StratNopeCustom::DirectSafe) => { override_nope = direct_attack; }
            Strategy::NopeCustom(StratNopeCustom::DirectUnsafe) => { override_nope = direct_attack && !hands[num].contains(&Card::Defuse); }
            Strategy::NopeCustom(StratNopeCustom::Wait) => {
                let few_players_left = hands.len() <= 2;
                let few_cards_left = Helpers::count_total_cards(hands) <= 8;
                override_nope = few_players_left || few_cards_left;
            }
            _ => {}
        }

        return override_nope;
    }
}