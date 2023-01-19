use std::collections::HashMap;
use enum_iterator::all;
use crate::{game::{Deck, Card, Color, Game}, strats::{Ideas, IdeaList, Idea, IdeaAction, IdeaWin, IdeaActionRating, IdeaRateCard, IdeaRateTank}, helpers::Helpers};

impl SimConfig
{
    pub fn new() -> Self
    {
        Self {
            file_prefix: "lala".to_string(),
            num_iterations: 100000,
            print_interval: 1000,
            player_count: 4,
            score_threshold_base: 10,
            score_threshold: 10,
            randomize_player_count: true,
            create_gamestate_video: false,
            print_gameplay: false,
            track_wins: true,
            track_per_player: false,
            track_start_cards: false,
            fixed: HashMap::new(),
            cards: Vec::new(),
            options: HashMap::new(),
            options_dict: HashMap::from([
                ("action".to_owned(), Idea::Action(IdeaAction::Pass)),
                ("action_rating".to_owned(), Idea::ActionRating(IdeaActionRating::Pass)),
                ("win_cond".to_owned(), Idea::Win(IdeaWin::Pass)),
                ("rate_tank_self".to_owned(), Idea::RateTank(IdeaRateTank::Pass)),
                ("rate_card_self".to_owned(), Idea::RateCard(IdeaRateCard::Pass)),
                ("rate_tank_other".to_owned(), Idea::RateTank(IdeaRateTank::Pass)),
                ("rate_card_other".to_owned(), Idea::RateCard(IdeaRateCard::Pass)),
            ])
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
        // this lists ALL strategies across all categories
        // then we grab slices to save them per category
        let fields_auto = all::<Idea>().collect::<Vec<_>>();
        let mut options:IdeaList = HashMap::new();
        for (k,v) in cfg.options_dict.iter()
        {
            options.insert(k.clone(), Helpers::create_enum_match_list(&fields_auto, *v));
        }
        return options;
    }

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
    pub options_dict: HashMap<String, Idea>
}
