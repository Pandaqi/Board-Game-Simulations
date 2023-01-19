use std::{fmt, collections::HashMap};
use enum_iterator::Sequence;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum Idea {
    Action(IdeaAction),
    ActionRating(IdeaActionRating),
    RateTank(IdeaRateTank),
    RateCard(IdeaRateCard),
    Win(IdeaWin)
}

impl fmt::Display for Idea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub type Ideas = HashMap<String, Idea>;
pub type IdeaList = HashMap<String, Vec<Idea>>;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum IdeaAction {
    Pass,
    Random,
    Score,
    Steal,
    PreferScore,
    PreferSteal,
    StealMatchThree,
    StealMatchTwo
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum IdeaActionRating {
    Pass,
    StealHigh,
    StealIfLead,
    StealIfBetter,
    StealIfClose
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum IdeaRateTank {
    Pass
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum IdeaRateCard {
    Pass,
    CountCard,
    CountColor
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum IdeaWin {
    Pass,
    StealOtherWinning,
    ScoreOtherWinning,
    ScoreSelfWinning,
    StealSelfWinning,
    StealIfBehind,
    ScoreIfBehind
}

