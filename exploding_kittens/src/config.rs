use crate::strats::{StratPlay, StratNope, StratCombo, StratKitten, StratVictim};

pub struct StratListStruct {
    pub play: Vec<StratPlay>,
    pub nope: Vec<StratNope>,
    pub combo: Vec<StratCombo>,
    pub kitten: Vec<StratKitten>,
    pub victim: Vec<StratVictim>
}

impl StratListStruct
{
    pub fn new() -> Self
    {
        Self {
            play: Vec::new(),
            nope: Vec::new(),
            combo: Vec::new(),
            kitten: Vec::new(),
            victim: Vec::new()
        }
    }
}

pub struct SimConfig {
    pub file_prefix: String,
    pub num_iterations: usize,
    pub player_count: usize,
    pub randomize_player_count: bool,
    pub create_images: bool,
    pub print_gameplay: bool,
    pub track_per_player: bool,
    pub print_interval: usize
}

impl SimConfig
{
    pub fn new() -> Self
    {
        Self {
            file_prefix: "third_test".to_string(),
            num_iterations: 1,
            print_interval: 1000,
            player_count: 4,
            randomize_player_count: true,
            create_images: false,
            print_gameplay: true,
            track_per_player: false,
        }
    }
}