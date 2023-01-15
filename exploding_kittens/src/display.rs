use fontdue::{*, layout::{TextStyle, LayoutSettings, Layout, CoordinateSystem, CharacterData}};
use raqote::*;

use crate::{strats::{Hand, Card}, helpers::CARD_DATA};

#[derive(Copy, Clone)]
struct Pos {
    x: f32,
    y: f32
}

pub struct Display {
    font:Font,
    font_size: f32,
}

impl Display
{
    pub fn new() -> Self
    {
        let font = Display::load_font().unwrap();
        Self {
            font,
            font_size: 12.0
        }
    }

    fn load_font() -> FontResult<Font> 
    {
        let font_data = include_bytes!("../assets/bebas_neue.otf") as &[u8];
        let settings = fontdue::FontSettings::default();
        fontdue::Font::from_bytes(font_data, settings)
    }

    fn rgb_to_u32(red: usize, green: usize, blue: usize, alpha: usize) -> u32 
    {
        let r = red.clamp(0, 255);
        let g = green.clamp(0, 255);
        let b = blue.clamp(0, 255);
        let a = alpha.clamp(0, 255);
        ((a << 24) | (r << 16) | (g << 8) | b) as u32
    }

    pub fn set_font_size(&mut self, font_size_px:f32)
    {
        self.font_size = font_size_px;
    }

    pub fn print_on_canvas(&self, dt:&mut DrawTarget, content:&String, x: f32, y: f32)
    {
        // prepare a layout for the text
        // (a text box: width, height, centering, font size)
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x,
            y,
            max_width: Option::Some(500_f32),
            max_height: Option::Some(200_f32),
            horizontal_align: fontdue::layout::HorizontalAlign::Center,
            ..LayoutSettings::default()
        });

        let fonts = [&self.font];
        layout.append(&fonts, &TextStyle::new(&content, self.font_size, 0));

        // now convert all glyps to images
        // which we add to the draw target
        for glyph in layout.glyphs() {
            let (metrics, coverage) = self.font.rasterize(glyph.parent, glyph.key.px);

            let mut image_data = Vec::with_capacity(coverage.len());
            for cov in coverage.iter() {
                let pixel = Display::rgb_to_u32(0, 0, 0, *cov as usize);
                image_data.push(pixel);
            }

            let mut pb = PathBuilder::new();
            pb.rect(glyph.x, glyph.y, metrics.width as f32, metrics.height as f32);
            //pb.rect(0.0, 0.0, metrics.width as f32, metrics.height as f32);
            let path = pb.finish();

            let source = Source::Image(
                Image {
                    width: metrics.width as i32,
                    height: metrics.height as i32,
                    data: &image_data,
                },
                Transform::create_translation(-glyph.x, -glyph.y)
            );

            dt.fill(&path, 
                &source,
                &DrawOptions::new()
            );
        }
    }

    pub fn save_gamestate_to_png(&self, turn_num:usize, hands:&Vec<Hand>, p_alive:&Vec<usize>)
    {
        let mut dt = DrawTarget::new(960, 540);

        let options = DrawOptions::new();

        // solid white background
        let mut pb = PathBuilder::new();
        pb.rect(0.0, 0.0, 960.0, 540.0);
        let path = pb.finish();

        let bg_color = SolidSource { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF };
        let source = Source::Solid(bg_color);
        dt.fill(&path, &source, &options);
        
        // display hands for living players (at fixed position)
        for i in 0..hands.len()
        {
            let player_num = p_alive[i];
            Display::draw_hand(&mut dt, player_num, &hands[i]);
        }

        
        // TESTING
        self.print_on_canvas(&mut dt,  &"TESTIE".to_owned(), 50.0, 50.0);
        

        let filename = "turn_".to_owned() + &format!("{:0>4}", turn_num.to_string());
        let filepath = "images_gamestate/".to_owned() + &filename + ".png";
        let result = dt.write_png(filepath);
        match result {
            Ok(_res) => println!("Saved image!"),
            Err(_err) => println!("Error saving image"),
        };
    }

    fn draw_hand(dt:&mut DrawTarget, position:usize, hand:&Hand)
    {
        let options = DrawOptions::new();
        if hand.len() <= 0 { return; }

        // positioned in a circle around the table
        let positions:Vec<Pos> = vec![
            Pos { x: 0.5*960.0, y: 0.15*540.0 },
            Pos { x: 0.85*960.0, y: 0.5*540.0 },
            Pos { x: 0.5*960.0, y: 0.85*540.0 },
            Pos { x: 0.15*960.0, y: 0.5*540.0 }
        ];

        let mut base_pos = positions[position].clone();

        let card_width:f32 = 25.0;
        let card_height:f32 = card_width*1.5;
        let margin:f32 = 2.0;

        let max_num_cols = 6;
        let num_cols = hand.len().min(max_num_cols);
        let num_rows = ((hand.len() as f32) / (num_cols as f32)).ceil() as usize;

        // to center the hand
        base_pos.x -= 0.5*((num_cols-1) as f32)*(card_width+margin);
        base_pos.y -= 0.5*((num_rows-1) as f32)*(card_height+margin);

        // for each card, draw a rectangle in the right color (offset to show all cards nicely)
        for (k,v) in hand.iter().enumerate()
        {
            let col = (k % num_cols) as f32;
            let row = ((k as f32) / (num_cols as f32)).floor();
            let x = base_pos.x + col*(card_width+margin);
            let y = base_pos.y + row*(card_height+margin);

            let mut pb = PathBuilder::new();
            pb.rect(x, y, card_width, card_height);
            let path = pb.finish();

            let source = Source::Solid(Display::get_card_color(v));
            dt.fill(&path, &source, &options);

            let is_cat_card = CARD_DATA[v].combo;
            if is_cat_card
            {
                let mut pb = PathBuilder::new();
                let dims:Pos = Display::get_cat_card_shape(v);
                pb.rect(
                    x+0.5*card_width-0.5*dims.x,
                    y+0.5*card_height-0.5*dims.y,
                    dims.x, 
                    dims.y
                );
                let path = pb.finish();
                let source = Source::Solid(SolidSource { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF });
                dt.fill(&path, &source, &options);
            }
        }
    }

    fn get_card_color(card:&Card) -> SolidSource
    {
        let col:SolidSource = match card
        {
            Card::Defuse => SolidSource { r: 0x0, g: 0xFF, b: 0x0, a: 0xFF }, // green
            Card::Kitten => SolidSource { r: 0x0, g: 0x0, b: 0x0, a: 0xFF }, // black
            Card::Attack => SolidSource { r: 0xFF, g: 0xFF, b: 0x0, a: 0xFF }, // yellow
            Card::Favor => SolidSource { r: 0x77, g: 0x77, b: 0x77, a: 0xFF }, // grey
            Card::Future => SolidSource { r: 0xFF, g: 0x0, b: 0xFF, a: 0xFF }, // pink
            Card::Skip => SolidSource { r: 0x0, g: 0x0, b: 0xFF, a: 0xFF }, // blue
            Card::Shuffle => SolidSource { r: 0xFF, g: 0x99, b: 0x33, a: 0xFF }, // brown
            Card::Nope => SolidSource { r: 0xFF, g: 0x0, b: 0x0, a: 0xFF }, // red
            _ => SolidSource { r: 0x0, g: 0x0, b: 0x0, a: 0xFF } // all combo cards have same color, but add an extra shape
        };
        return col;
    }

    fn get_cat_card_shape(card:&Card) -> Pos
    {
        let shape:Pos = match card
        {
            Card::Beardcat => Pos { x: 5.0, y: 5.0 },
            Card::Cattermelon => Pos { x: 10.0, y: 10.0 },
            Card::Potatocat => Pos { x: 15.0, y: 5.0 },
            Card::Rainbowcat => Pos { x: 5.0, y: 15.0 },
            Card::Tacocat => Pos { x: 15.0, y: 15.0 },
            _ => { Pos { x: 1.0, y: 1.0 } }
        };
        return shape;
    }
}