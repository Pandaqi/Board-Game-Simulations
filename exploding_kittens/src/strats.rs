use std::fmt;

use enum_iterator::Sequence;
use rand::seq::SliceRandom;

use crate::config::StratListStruct;

pub type Combo = (Card, usize);

// The strategy a specific player is using
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
    ThreesAlways
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
    Random
}

// TO DO: figure out a way to make this call ONCE for all appropriate enums
impl fmt::Display for StratPlay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for StratNope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for StratCombo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for StratKitten {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for StratVictim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}