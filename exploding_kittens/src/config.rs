use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::strats::*;
pub struct SimConfig {
    pub file_prefix: String,
    pub num_iterations: usize,
    pub player_count: usize,
    pub randomize_player_count: bool,
    pub create_gamestate_video: bool,
    pub print_gameplay: bool,
    pub track_wins: bool,
    pub track_per_player: bool,
    pub track_start_cards: bool,
    pub print_interval: usize,
    pub fixed_strat: Strat
}

lazy_static! {
    pub static ref CONFIG:SimConfig = SimConfig 
    {
        file_prefix: "final".to_string(),
        num_iterations: 1,
        print_interval: 1000,
        player_count: 4,
        randomize_player_count: false,
        create_gamestate_video: true,
        print_gameplay: true,
        track_wins: false,
        track_per_player: false,
        track_start_cards: false,
        fixed_strat: HashMap::from([
            ("answer".to_owned(), Strategy::Answer(StratAnswer::Always)),

            ("combo".to_owned(), Strategy::Combo(StratCombo::Sometimes)),
            ("combo_pref".to_owned(), Strategy::ComboPref(StratComboPref::Two)),
            ("combo_type".to_owned(), Strategy::ComboType(StratComboType::CatMost)),

            ("nope".to_owned(), Strategy::Nope(StratNope::Always)),
            ("nope_defend".to_owned(), Strategy::NopeDefend(StratNopeDefend::Always)),
            ("nope_custom".to_owned(), Strategy::NopeCustom(StratNopeCustom::DirectSafe)),

            ("future".to_owned(), Strategy::Future(StratFuture::Always)),
            ("kitten".to_owned(), Strategy::Kitten(StratKitten::Top)),
            ("play".to_owned(), Strategy::Play(StratPlay::AggroStart)),
            ("victim".to_owned(), Strategy::Victim(StratVictim::DefuseProb)),
        ])
    };
}

// Use this FFMPEG command to turn your sequence of images (from "create_gamestate_video") into a video
// ffmpeg -r 1 -f image2 -s 960x540 -i turn_%04d.png -vcodec libx264 -crf 25 -pix_fmt yuv420p test.mp4
//
// -r = framerate