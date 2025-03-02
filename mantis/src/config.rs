use std::collections::HashMap;
use crate::{game::{Deck, Card, Color, Game}, strats::{Ideas, IdeaList, Idea, IdeaAction, IdeaWin, IdeaActionRating, IdeaRateCard, IdeaRateTank}, helpers::Helpers};

// Use this FFMPEG command to turn your sequence of images (from "create_gamestate_video") into a video (-r = framerate)
// ffmpeg -r 1/3 -f image2 -s 960x540 -i turn_%04d.png -vcodec libx264 -crf 25 -pix_fmt yuv420p test.mp4


impl SimConfig
{
    pub fn new() -> Self
    {
        Self {
            file_prefix: "video_5".to_string(),
            num_iterations: 1,
            print_interval: 1000,
            double_strategy: true,
            player_count: 4,
            score_threshold_base: 10,
            score_threshold: 10,
            randomize_player_count: false,
            create_gamestate_video: true,
            print_gameplay: false,
            track_wins: false,
            track_per_player: true, // if true, gives 0 player the "fixed" strategy below, and only provides one graph for wins per PLAYER
            track_start_cards: false, // @TODO: does nothing yet
            fixed: HashMap::from([
                ("offset".to_owned(), 5),
                ("other_color_match".to_owned(), -5),
                ("other_card_match".to_owned(), -4), 
                ("other_card".to_owned(), -5),
                ("other_color".to_owned(), -2),
                ("other_small_stack".to_owned(), 0),
                ("other_biggest_stack".to_owned(), -5),
                ("self_color_match".to_owned(), 5),
                ("self_card_match".to_owned(), 5), 
                ("self_card".to_owned(), 5),
                ("self_color".to_owned(), 5),
                ("self_small_stack".to_owned(), 5),
                ("self_biggest_stack".to_owned(), 5),
            ]),
            cards: Vec::new(),
            options: HashMap::new(),
            options_dict: HashMap::from([

                ("other_color_match".to_owned(), (-5..=5).collect()),
                ("other_card_match".to_owned(), (-5..=5).collect()), 
                ("other_card".to_owned(), (-5..=5).collect()),
                ("other_color".to_owned(), (-5..=5).collect()),
                ("other_small_stack".to_owned(), (-5..=5).collect()),
                ("other_biggest_stack".to_owned(), (-5..=5).collect()),
                ("offset".to_owned(), (-5..=5).collect()),
            ])

            /*options_dict: HashMap::from([

                // points per color that matches (the back of the top card)
                ("other_color_match".to_owned(), (-1..=1).collect()),

                // points per card that matches (the back of the top card),
                ("other_card_match".to_owned(), (-1..=1).collect()), 

                // points per card you possess
                ("other_card".to_owned(), (-1..=1).collect()),

                // points per (unique) color you possess
                ("other_color".to_owned(), (-1..=1).collect()),

                // points if this player is winning ( = first place)
                // or losing ( = last place)
                ("other_winner".to_owned(), (-1..=1).collect()),
                ("other_loser".to_owned(), (-1..=1).collect()),

                // points if this player is ABOUT to win ( = score within a few of the threshold)
                ("other_near_win".to_owned(), (-1..=1).collect()),

                // points for how many small/big stacks you have (<=1, >=3)
                ("other_small_stack".to_owned(), (-1..=1).collect()),
                ("other_big_stack".to_owned(), (-1..=1).collect()),

                // points for the biggest matching stack 
                ("other_biggest_stack".to_owned(), (-1..=1).collect()),

                // points for matching two colours
                ("other_match_two".to_owned(), (-1..=1).collect()),

                // a fixed numerical offset from all your own scores (higher value = more likely to score)
                ("offset".to_owned(), (-1..=1).collect()),

                // how to value a guaranteed steal (three matching colors)
                ("guaranteed_steal".to_owned(), (-1..=1).collect()),

                // points for a player who is close to you in score
                ("close_score".to_owned(), (-1..=1).collect()),

                // points for a player who is better than you in score
                ("better_score".to_owned(), (-1..=1).collect()),

                // points for a player who has a significant lead
                ("lead_score".to_owned(), (-1..=1).collect()),

                // points for override strategy (-4 = always score, 4 = random, rest is pass)
                // "always steal" is impossible, as you need to score to win
                // @DEBUGGING/@TODO: this is not set to -4,4 to just completely ignore this one
                ("override".to_owned(), (-3..=3).collect()),
                
            ])*/
        }
    }
    
    pub fn randomize_player_count(cfg:&mut SimConfig)
    {
        if !cfg.randomize_player_count { return; }
        cfg.player_count = Game::get_random_player_count();
        
        if cfg.player_count == 2
        {
            cfg.score_threshold = (cfg.score_threshold_base as f64 * 1.5).round() as usize;
        }
    }

    pub fn generate_all_strats(cfg:&SimConfig) -> IdeaList
    {
        let mut arr:IdeaList = HashMap::new();
        for (k,v) in cfg.options_dict.iter()
        {
            arr.insert(k.clone(), v.clone());

            if cfg.double_strategy && &k[..5] == "other"
            {
                arr.insert(k.clone().replace("other", "self"), v.clone());
            }
        }
        return arr;
    }

    /*
    pub fn generate_all_strats(cfg:&SimConfig) -> IdeaList
    {
        // in this case, `options_dict` is a HashMap with <String, OneEnumValueOfCategory>
        // list ALL strategies across all categories; then grab slices to save them per category
        let fields_auto = all::<Idea>().collect::<Vec<_>>();
        let mut options:IdeaList = HashMap::new();
        for (k,v) in cfg.options_dict.iter()
        {
            options.insert(k.clone(), Helpers::create_enum_match_list(&fields_auto, *v));
        }
        return options;
    }
     */

    pub fn generate_all_cards() -> Vec<Card>
    {
        let num_colors = 7;
        let mut arr:Vec<Card> = Vec::new();
        for i in 0..num_colors
        {
            let options = SimConfig::generate_unique_color_pairs(num_colors, i);
            for v in options.iter()
            {
                arr.push(Card::new(i, v.0, v.1));
            }
        }
        return arr;
    }

    pub fn generate_unique_color_pairs(num_colors:u8, ignore:Color) -> Vec<(Color, Color)>
    {
        let mut arr:Vec<(Color, Color)> = Vec::new();
        for a in 0..num_colors
        {
            if ignore == a { continue; }
            for b in (a+1)..num_colors 
            {
                if ignore == b { continue; }
                arr.push((a as Color, b as Color));
            }
        }
        return arr;
    }

    pub fn test_setup(&mut self, player_count:usize)
    {
        self.player_count = player_count;
        self.cards = SimConfig::generate_all_cards();
        self.options = SimConfig::generate_all_strats(&self);
    }
}

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
    pub fixed: Ideas,
    pub cards: Deck,
    pub score_threshold_base: usize,
    pub score_threshold: usize,
    pub options: IdeaList,
    pub options_dict: HashMap<String, Vec<i32>>,
    pub double_strategy: bool,
}
