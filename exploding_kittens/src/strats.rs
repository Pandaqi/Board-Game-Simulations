use std::{fmt, collections::HashMap};

use enum_iterator::Sequence;
pub type Combo = (Card, usize);

// NOTE: This is an enum that has subenums. 
// It makes syntax a little more verbose / complicated throughout the simulation, so why use it?
// 1) One type for all strategies. 
// 2) This easily allows generic functions on it => such as the impl below that allows printing enums as strings
// 3) And HashMaps, which are iterable => no duplicate code or fixed indexing
// 4) Clarity => I know all strategies can always be found in the enum Strategy

// Maybe, when I get more experience with Rust, I find better ways.

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum Strategy {
    Play(StratPlay),
    Nope(StratNope),
    Combo(StratCombo),
    ComboPref(StratComboPref),
    ComboType(StratComboType),
    Kitten(StratKitten),
    Victim(StratVictim),
    Answer(StratAnswer),
    Future(StratFuture)
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub type Strat = HashMap<String, Strategy>;
pub type StratList = HashMap<String, Vec<Strategy>>;

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

/* PLAY ( = picking cards to play) */
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratPlay {
    Random,
    Never,
    One,
    All,
    OnlyAfterKitten,
    NotIfSafe
}

// How you react to the player before you exploding (and putting a kitten back in the deck)
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratAnswer {
    Random,
    Ignore,
    FewPlayers,
    Defuseless,
    Always
}

// How you deal with future cards (asking and using)
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratFuture {
    Random,
    Never,
    Defuseless,
    Changable,
    Always
}

/* NOPE */
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

/* COMBOS */
// The probability of asking and playing a combo
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratCombo {
    Never = 0,
    Rarely = 3,
    Sometimes = 7,
    Often = 11,
    Always = 15
}

// Whether you prefer combos of 2 or 3
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratComboPref {
    Two = 0,
    TwoMost = 3,
    Split = 7,
    ThreeMost = 11,
    Three = 15
}

// Whether you use cat cards for combos or all cards
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum StratComboType {
    Cat = 0,
    CatMost = 3,
    Split = 7,
    AllMost = 11,
    All = 15
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
    Anti,
    MostCards,
    LeastCards,
    SeatBefore,
    SeatAfter,
    PickOne,
    PickDiverse
}
