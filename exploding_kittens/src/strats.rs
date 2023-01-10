use std::{fmt, collections::HashMap};

use enum_iterator::Sequence;
pub type Combo = (Card, usize);

// The strategy a specific player is using
/*
#[derive(Copy, Clone, Debug)]
pub struct Strat {
    pub play: StratPlay,
    pub nope: StratNope,
    pub combo: StratCombo,
    pub kitten: StratKitten,
    pub victim: StratVictim
}

impl Strat
{
    pub fn new_random(options:&StratListStruct) -> Self
    {
        let mut rng = rand::thread_rng();
        Strat {
            play: *options.play.choose(&mut rng).unwrap(),
            nope: *options.nope.choose(&mut rng).unwrap(),
            combo: *options.combo.choose(&mut rng).unwrap(),
            kitten: *options.kitten.choose(&mut rng).unwrap(),
            victim: *options.victim.choose(&mut rng).unwrap()
        }
    }
}
 */

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum Strategy {
    Play(StratPlay),
    Nope(StratNope),
    Combo(StratCombo),
    Kitten(StratKitten),
    Victim(StratVictim)
}

pub type Strat = HashMap<String, Strategy>;
pub type StratList = HashMap<String, Vec<Strategy>>;

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}


#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Card {
    Defuse,
    Kitten,
    Nope,
    Attack,
    Skip,
    Favor,
    Shuffle,
    Future,
    Cattermelon,
    Beardcat,
    Potatocat,
    Rainbowcat,
    Tacocat
}

pub type Hand = Vec<Card>;

pub struct CardData {
    pub freq: usize,
    pub combo: bool,
    pub play: bool,
    pub anti: bool
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratPlay {
    Random,
    Never,
    One,
    All,
    OnlyAfterKitten,
    NotIfSafe
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratNope {
    Random,
    Never,
    Rarely,
    Sometimes,
    Often,
    Always,
    OnlyIfSafe,
    OnlyDefuseless,
    Direct,
    DirectUnsafe,
    Wait,
    DeNope,
    DeNopeDirect
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratCombo {
    Random,
    Never,
    Rarely,
    Sometimes,
    Often,
    Always,
    ThreesSometimes,
    ThreesAlways,
    AllCards,
    AllCardsThrees
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratKitten {
    Random,
    Top,
    TopSecond,
    TopFourth,
    TopCond,
    Bottom
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratVictim {
    Random,
    Defuse,
    DefuseProb,
    MostCards,
    LeastCards,
    SeatBefore,
    SeatAfter,
    PickOne,
    PickDiverse
}