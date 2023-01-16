use crate::{helpers::{Helpers}, config::{SimConfig, CONFIG}, strats::{StratList, Card}};

use plotters::{prelude::*};
use std::{ops::Range};

pub struct SimResults {
    pub wins_per_player: Vec<usize>,
    pub winning_cards: Vec<Card>,
    pub options: StratList,
    pub strats: StratList,
}

pub struct Results {}

impl Results
{
    pub fn display(res:SimResults)
    {

        if CONFIG.track_per_player 
        {
            let player_wins = Helpers::to_string_list(&res.wins_per_player);
            let player_options = Helpers::to_string_list(&(0..CONFIG.player_count).collect());
            Results::to_histogram("per_player_check", player_wins, player_options);
        }

        if CONFIG.track_start_cards 
        {
            let winning_cards = Helpers::compensate_for_card_frequency(&res.winning_cards);
            let card_wins = Helpers::to_string_list(&winning_cards);
            let card_options = Helpers::to_string_list(&Helpers::get_all_possible_cards());
            Results::to_histogram("per_starting_card", card_wins, card_options);
        }

        if CONFIG.track_wins
        {
            for(k,v) in res.options.iter()
            {
                let strats = Helpers::extract_inside_parentheses(Helpers::to_string_list(res.strats.get(k).unwrap()));
                let options = Helpers::extract_inside_parentheses(Helpers::to_string_list(v));
                Results::to_histogram(&k, strats, options);
            }
        }
    }
      
    fn to_histogram(file_key:&str, data:Vec<String>, x_values:Vec<String>)
    {
        let upper_bound = (4.0 * (data.len() as f64) / (x_values.len() as f64)) as i32;
        let y_values:Range<i32> = 0..upper_bound;

        let file_path = "images/".to_owned() + &CONFIG.file_prefix.to_owned() + "_" + file_key + ".png";
        let root_area = BitMapBackend::new(&file_path, (900, 600)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();
    
        let mut graph_title = "Wins per ".to_owned() + file_key + " strategy";
        if CONFIG.track_per_player { graph_title = "Wins per player".to_owned(); }
        if CONFIG.track_start_cards { graph_title = "Wins per start card".to_owned(); }

        let mut ctx = ChartBuilder::on(&root_area)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption(graph_title, ("serif", 40))
            .build_cartesian_2d(x_values.into_segmented(), y_values)
            .unwrap();
    
        ctx.configure_mesh()
            .x_labels(x_values.len()+1)
            .draw()
            .unwrap();
    
        ctx.draw_series(
            Histogram::vertical(&ctx)
            .margin(5)
            .data(data.iter().map(|x| (x, 1)))
        ).unwrap();
    }
}