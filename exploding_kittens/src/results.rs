use crate::{helpers::Helpers, config::{SimConfig, StratListStruct}};

use plotters::{prelude::*};
use std::{ops::Range};

pub struct SimResults {
    pub wins_per_player: Vec<usize>,
    pub options: StratListStruct,
    pub strats: StratListStruct,
}

pub struct Results {}

impl Results
{
    pub fn display(cfg: &SimConfig, res:SimResults)
    {
        let play_strats = Helpers::to_string_list(&res.strats.play);
        let nope_strats = Helpers::to_string_list(&res.strats.nope);
        let combo_strats = Helpers::to_string_list(&res.strats.combo);
        let kitten_strats = Helpers::to_string_list(&res.strats.kitten);
        let victim_strats = Helpers::to_string_list(&res.strats.victim);
    
        let play_options = Helpers::to_string_list(&res.options.play);
        let nope_options = Helpers::to_string_list(&res.options.nope);
        let combo_options = Helpers::to_string_list(&res.options.combo);
        let kitten_options = Helpers::to_string_list(&res.options.kitten);
        let victim_options = Helpers::to_string_list(&res.options.victim);

        let player_wins = Helpers::to_string_list(&res.wins_per_player);
        let player_options = Helpers::to_string_list(&vec![0,1,2,3]); // TO DO: update to actually used player count

        // TO DO: now we just cast the numeric options to strings as well ... is that right? 
        // Need to figure out how to use Plotters library for histograms of numbers and booleans as well

        if cfg.create_images {
            if cfg.track_per_player {
                Results::to_histogram(cfg, "per_player_check", player_wins, player_options);
            } else {
                Results::to_histogram(cfg, "play", play_strats, play_options);
                Results::to_histogram(cfg, "nope", nope_strats, nope_options);
                Results::to_histogram(cfg, "combo", combo_strats, combo_options);
                Results::to_histogram(cfg, "kitten", kitten_strats, kitten_options);
                Results::to_histogram(cfg, "victim", victim_strats, victim_options);
            }
        }
    }
    
    
    fn to_histogram(cfg:&SimConfig, file_key:&str, data:Vec<String>, x_values:Vec<String>)
    {
        let upper_bound = 2*( (cfg.num_iterations as f64) / (x_values.len() as f64) ).ceil() as i32;
        let y_values:Range<i32> = 0..upper_bound;
    
        let file_path = "images/".to_owned() + &cfg.file_prefix.to_owned() + "_" + file_key + ".png";
        let root_area = BitMapBackend::new(&file_path, (900, 600)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();
    
        let mut graph_title = "Wins per ".to_owned() + file_key + " strategy";
        if cfg.track_per_player { graph_title = "Wins per player".to_owned(); }

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