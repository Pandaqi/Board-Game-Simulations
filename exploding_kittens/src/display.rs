use fontdue::{*, layout::{TextStyle, LayoutSettings, Layout, CoordinateSystem, CharacterData}};
//use raqote::*;
use tiny_skia::*;

use crate::{strats::{Hand, Card}, helpers::CARD_DATA, config::CONFIG, game::GameState};

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

    pub fn print_on_canvas(&self, dt:&mut Pixmap, content:&String, x: f32, y: f32, width: f32)
    {
        // prepare a layout for the text
        // (a text box: width, height, centering, font size)
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x,
            y,
            max_width: Option::Some(width),
            max_height: Option::Some(self.font_size+2.0),
            horizontal_align: fontdue::layout::HorizontalAlign::Center,
            ..LayoutSettings::default()
        });

        let fonts = [&self.font];
        layout.append(&fonts, &TextStyle::new(&content, self.font_size, 0));

        // now convert all glyps to images
        // which we add to the draw target
        for glyph in layout.glyphs() {
            if glyph.parent == ' ' { continue; }

            let (metrics, coverage) = self.font.rasterize(glyph.parent, glyph.key.px);

            let mut glyph_pixmap:Pixmap = Pixmap::new(metrics.width as u32, metrics.height as u32).unwrap();
            //let mut image_data = Vec::with_capacity(coverage.len());
            for (idx, cov) in coverage.iter().enumerate()
            {
                let x = idx % metrics.width;
                let y = ((idx as f64) / (metrics.width as f64)).floor() as usize;

                let mut paint = Paint::default();
                paint.set_color(Color::from_rgba8(0,0,0,*cov as u8));

                glyph_pixmap.fill_rect(
                    Rect::from_xywh(x as f32, y as f32, 1.0, 1.0).unwrap(),
                    &paint,
                    Transform::identity(),
                    None
                );
            }

            let mut paint = PixmapPaint::default();
            paint.quality = FilterQuality::Bicubic;

            dt.draw_pixmap(
                glyph.x as i32, 
                glyph.y as i32, 
                glyph_pixmap.as_ref(), 
                &paint, 
                Transform::identity(), 
                None
            );
        }
    }

    pub fn save_gamestate_to_png(&self, turn_num:usize, hands:&Vec<Hand>, p_alive:&Vec<usize>, p_cur:usize, state:&GameState)
    {
        if !CONFIG.create_gamestate_video { return; }

        let mut dt = Pixmap::new(960, 540).unwrap();

        // solid white background
        dt.fill(Color::from_rgba8(0xFF, 0xFF, 0xFF, 0xFF));
        
        // display hands for living players (at fixed position)
        for i in 0..hands.len()
        {
            let player_num = p_alive[i];
            let is_active = p_cur == i;
            self.draw_hand(&mut dt, player_num, &hands[i], is_active);
        }

        // display turn summary (in center of screen)
        if state.turn_summary.len() > 0
        {
            let text_width = 150.0;
            let text_height = self.font_size;
            let text_x = 0.5*960.0 - 0.5*text_width;
            let mut text_y = 0.5*540.0 - 0.5*(state.turn_summary.len() as f32)*text_height;

            for v in state.turn_summary.iter()
            {
                self.print_on_canvas(&mut dt, &v, text_x, text_y, text_width);
                text_y += text_height;
            }
        }

        let filename = "turn_".to_owned() + &format!("{:0>4}", turn_num.to_string());
        let filepath = "images_gamestate/".to_owned() + &filename + ".png";
        let result = dt.save_png(filepath);
        match result {
            Ok(_res) => println!("Saved image!"),
            Err(_err) => println!("Error saving image"),
        };
    }

    fn draw_hand(&self, dt:&mut Pixmap, position:usize, hand:&Hand, is_active:bool)
    {
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
        let num_cols = hand.len().min(max_num_cols) as f32;
        let num_rows = ((hand.len() as f32) / (num_cols as f32)).ceil() as f32;

        // to center the hand
        base_pos.x -= 0.5*num_cols*(card_width+margin);
        base_pos.y -= 0.5*num_rows*(card_height+margin);

        if is_active
        {
            let max_width = num_cols.max(3.0) * (card_width+margin);
            let x = base_pos.x.clamp(10.0, 950.0);
            let mut y = base_pos.y + num_rows*(card_height+margin) + self.font_size;
            if base_pos.y > self.font_size { y = base_pos.y - self.font_size; }
            self.print_on_canvas(dt,  &"Turn".to_owned(), x, y, max_width);
        }

        // for each card, draw a rectangle in the right color (offset to show all cards nicely)
        for (k,v) in hand.iter().enumerate()
        {
            let col = (k % (num_cols as usize)) as f32;
            let row = ((k as f32) / num_cols).floor();
            let x = base_pos.x + col*(card_width+margin);
            let y = base_pos.y + row*(card_height+margin);

            let mut paint = Paint::default();
            paint.set_color(Display::get_card_color(v));
            paint.anti_alias = true;

            let path = PathBuilder::from_rect(Rect::from_xywh(x, y, card_width, card_height).unwrap());

            dt.fill_path(
                &path, 
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None
            );

            let is_cat_card = CARD_DATA[v].combo;
            if is_cat_card
            {
                let dims:Pos = Display::get_cat_card_shape(v);
                let path = PathBuilder::from_rect(Rect::from_xywh(
                    x+0.5*card_width-0.5*dims.x,
                    y+0.5*card_height-0.5*dims.y,
                    dims.x, 
                    dims.y
                ).unwrap());

                let mut paint = Paint::default();
                paint.set_color(Color::from_rgba8(0xFF, 0xFF, 0xFF, 0xFF));
                paint.anti_alias = true;
                dt.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
            }
        }
    }

    fn get_card_color(card:&Card) -> Color
    {
        let col:Color = match card
        {
            Card::Defuse => Color::from_rgba8(0x0, 0xFF, 0x0, 0xFF), // green
            Card::Kitten => Color::from_rgba8(0x0, 0x0, 0x0, 0xFF), // black
            Card::Attack => Color::from_rgba8(0xFF, 0xFF, 0x0, 0xFF), // yellow
            Card::Favor => Color::from_rgba8(0x77, 0x77, 0x77, 0xFF), // grey
            Card::Future => Color::from_rgba8(0xFF, 0x0, 0xFF, 0xFF), // pink
            Card::Skip => Color::from_rgba8(0x0, 0x0, 0xFF, 0xFF), // blue
            Card::Shuffle => Color::from_rgba8(0xFF, 0x99, 0x33, 0xFF), // brown
            Card::Nope => Color::from_rgba8(0xFF, 0x0, 0x0, 0xFF), // red
            _ => Color::from_rgba8(0x0, 0x0, 0x0, 0xFF) // all combo cards have same color, but add an extra shape
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