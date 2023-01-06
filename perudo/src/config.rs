use crate::{guesser::{StratInitial, StratGuess, StratBluff}, challenger::StratChall};

pub struct StratListStruct {
    pub bluff: Vec<StratBluff>,
    pub init: Vec<StratInitial>,
    pub guess: Vec<StratGuess>,
    pub prev_player: Vec<f64>,
    pub next_player: Vec<f64>,
    pub chall_offset: Vec<i32>
}

pub struct SimConfig {
    pub file_prefix: String,
    pub num_iterations: usize,
    pub player_count: usize,
    pub randomize_player_count: bool,
    pub continue_until_winner: bool,
    pub use_no_perudo_round: bool, // "palafico": the round immediately after a player reaching 1 dice, aces don't count
    pub use_palafico_guessing: bool, // "palafico": in such a round, you also can't change the VALUE, only the NUMBER (most people don't know this rule, so it's optional)
    pub strats: StratListStruct,
}