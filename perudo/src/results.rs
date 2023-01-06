use crate::{helpers::Helpers, config::{SimConfig, StratListStruct}, guesser::{StratInitial, StratGuess}, challenger::StratChall};

use plotters::{prelude::*, coord::ranged1d::AsRangedCoord};
use std::{ops::Range, collections::HashMap};

pub struct SimResults {
    pub strats: StratListStruct,
}

pub struct Results {}

impl Results
{
    pub fn display(cfg: &SimConfig, res:SimResults)
    {
        let bluff_strats = Helpers::to_string_list(&res.strats.bluff);
        let init_strats = Helpers::to_string_list(&res.strats.init);
        let guess_strats = Helpers::to_string_list(&res.strats.guess);
        let prev_player_strats = Helpers::to_string_list(&res.strats.prev_player);
        let next_player_strats = Helpers::to_string_list(&res.strats.next_player);
        let chall_offset_strats = Helpers::to_string_list(&res.strats.chall_offset);
    
        let bluff_options = Helpers::to_string_list(&cfg.strats.bluff);
        let init_options = Helpers::to_string_list(&cfg.strats.init);
        let guess_options = Helpers::to_string_list(&cfg.strats.guess);
        let prev_player_options = Helpers::to_string_list(&cfg.strats.prev_player);
        let next_player_options = Helpers::to_string_list(&cfg.strats.next_player);
        let chall_offset_options = Helpers::to_string_list(&cfg.strats.chall_offset);

        // TO DO: now we just cast the numeric options to strings as well ... is that right?

        Results::to_histogram(cfg, "bluff", bluff_strats, bluff_options);
        Results::to_histogram(cfg, "init", init_strats, init_options);
        Results::to_histogram(cfg, "guess", guess_strats, guess_options);
        Results::to_histogram(cfg, "prev_player", prev_player_strats, prev_player_options);
        Results::to_histogram(cfg, "next_player", next_player_strats, next_player_options);
        Results::to_histogram(cfg, "chall_offset", chall_offset_strats, chall_offset_options);
    }
    
    
    fn to_histogram(cfg:&SimConfig, file_key:&str, data:Vec<String>, x_values:Vec<String>)
    {
        let upper_bound = 2*( (cfg.num_iterations as f64) / (x_values.len() as f64) ).ceil() as i32;
        let y_values:Range<i32> = 0..upper_bound;
    
        let file_path = "images/".to_owned() + &cfg.file_prefix.to_owned() + "_" + file_key + ".png";
        let root_area = BitMapBackend::new(&file_path, (600, 400)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();
    
        let graph_title = "Losses per ".to_owned() + file_key + " strategy";

        let mut ctx = ChartBuilder::on(&root_area)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption(graph_title, ("serif", 40))
            .build_cartesian_2d(x_values.into_segmented(), y_values)
            .unwrap();
    
        ctx.configure_mesh().draw().unwrap();
    
        ctx.draw_series(
            Histogram::vertical(&ctx)
            .margin(5)
            .data(data.iter().map(|x| (x, 1)))
        ).unwrap();
    
    }
}