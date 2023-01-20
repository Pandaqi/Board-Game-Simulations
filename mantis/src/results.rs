use plotters::{prelude::*};
use std::{ops::Range};

use crate::{game::Deck, strats::IdeaList, config::SimConfig, helpers::Helpers};

pub struct SimResults {
    pub wins_per_player: Vec<i32>,
    pub options: IdeaList,
    pub strats: IdeaList,
}

pub struct Results {}

impl Results
{
    pub fn display(res:SimResults)
    {
        let cfg = SimConfig::new();
        if cfg.track_per_player 
        {
            let player_wins = res.wins_per_player.clone();
            let player_options = (0..(cfg.player_count as i32)).collect();
            Results::to_histogram(&cfg, "per_player_check", player_wins, player_options);
        }

        if cfg.track_wins
        {
            for(k,v) in res.options.iter()
            {
                let strats = res.strats.get(k).unwrap().clone();
                let options = v.clone();
                Results::to_histogram(&cfg, &k, strats, options);
            }
        }
    }
      
    fn to_histogram(cfg:&SimConfig, file_key:&str, data:Vec<i32>, x_values:Vec<i32>)
    {
        let zoom_out = 2.0; // often seems best, 3.0 or 4.0 too far away
        let upper_bound = (zoom_out * (data.len() as f64) / (x_values.len() as f64)) as i32;
        let y_values:Range<i32> = 0..upper_bound;

        let file_path = "images/".to_owned() + &cfg.file_prefix.to_owned() + "_" + file_key + ".png";
        let root_area = BitMapBackend::new(&file_path, (900, 600)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();
    
        let mut graph_title = "Wins per ".to_owned() + file_key + " strategy";
        if cfg.track_per_player { graph_title = "Wins per player".to_owned(); }
        if cfg.track_start_cards { graph_title = "Wins per start card".to_owned(); }

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
            .style(BLUE.mix(0.85).filled())
            .data(data.iter().map(|x| (x, 1)))
        ).unwrap();
    }
}