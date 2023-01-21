use fontdue::{*, layout::{TextStyle, LayoutSettings, Layout, CoordinateSystem}};
use tiny_skia::*;

use crate::{game::{Hand, State, Action}, config::SimConfig, helpers::Helpers};

#[derive(Copy, Clone)]
struct Pos {
    x: f32,
    y: f32
}

pub struct Display {
    font:Option<Font>,
    font_size: f32,
    card_width: f32,
    card_height: f32,
    margin: f32
}

impl Display
{
    pub fn new(cfg:&SimConfig) -> Self
    {
        let font = if cfg.create_gamestate_video { Display::load_font() } else { None };
        Self {
            font,
            font_size: 12.0,
            card_width: 20.0,
            card_height: 20.0*1.5,
            margin: 6.0,
        }
    }

    pub fn is_enabled(&self) -> bool
    {
        return self.font.is_some();
    }

    fn load_font() -> Option<Font> 
    {
        let font_data = include_bytes!("../assets/ridley_grotesk_bold.otf") as &[u8];
        let settings = fontdue::FontSettings::default();
        let result = fontdue::Font::from_bytes(font_data, settings);
        return match result {
            Ok(res) => Some(res),
            Err(_err) => None
        }
    }

    pub fn set_font_size(&mut self, font_size_px:f32)
    {
        self.font_size = font_size_px;
    }

    pub fn print_on_canvas(&self, dt:&mut Pixmap, content:&String, x: f32, y: f32, width: f32, base_col:Color)
    {
        if self.font.is_none() { return; }

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

        let fonts = [self.font.as_ref().unwrap()];
        layout.append(&fonts, &TextStyle::new(&content, self.font_size, 0));

        let mut color = base_col.clone();

        // now convert all glyps to images
        // which we add to the draw target
        for glyph in layout.glyphs() {
            if glyph.parent == ' ' { continue; }

            let (metrics, coverage) = self.font.as_ref().unwrap().rasterize(glyph.parent, glyph.key.px);

            let mut glyph_pixmap:Pixmap = Pixmap::new(metrics.width as u32, metrics.height as u32).unwrap();
            //let mut image_data = Vec::with_capacity(coverage.len());
            for (idx, cov) in coverage.iter().enumerate()
            {
                let x = idx % metrics.width;
                let y = ((idx as f64) / (metrics.width as f64)).floor() as usize;

                let mut paint = Paint::default();
                color.set_alpha(*cov as f32 / 255.0);
                paint.set_color(color);

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

    pub fn save_strategies_to_png(&self, state:&mut State)
    {
        if !self.is_enabled() { return; }

        let mut dt = self.new_pixmap();
        let positions = Display::get_circle_positions(0.35);

        let key_match = vec!["other_color_match", "other_card_match", "other_card", "other_color", "other_winner", "other_loser", "offset", "guaranteed_steal", "override"];

        let text_col = Color::from_rgba8(0,0,0,0xFF);

        for i in 0..state.player_count
        {
            let text_width = 200.0;
            let text_height = self.font_size;
            let text_x = positions[i].x - 0.5*text_width;
            let mut text_y = positions[i].y - 0.5*(key_match.len() as f32)*text_height;

            for (k,v) in state.strategies[i].iter()
            {
                if !key_match.contains(&&k[..]) { continue; }
                let strat_name = k.replace("other_", "").replace("self_", "");
                let txt:String = strat_name + ": " + &v.to_string(); 
                self.print_on_canvas(&mut dt, &txt, text_x, text_y, text_width, text_col);
                text_y += text_height;
            }
        }

        self.save_png(dt, state);
    }
    
    pub fn save_gamestate_to_png(&mut self, state:&mut State, turn_done:bool)
    {
        if !self.is_enabled() { return; }

        let mut dt = self.new_pixmap();

        if !turn_done {
            self.draw_top_card(&mut dt, state);
            self.draw_player_ratings(&mut dt, state);
        } else {
            self.draw_turn_result(&mut dt, state);
        }

        // draw player hands
        for i in 0..state.hands.len()
        {
            self.draw_hand(&mut dt, i, &state, turn_done);
        }

        self.save_png(dt, state);
    }

    fn draw_turn_result(&mut self, dt:&mut Pixmap, state:&mut State)
    {
        let old_font_size = self.font_size;
        let text_col = Color::from_rgba8(0,0,0,0xFF);
        let text_width = 200.0;
        let mut content:String = if state.action_taken == Action::Score { "Score!".to_owned() } else { "Steal!".to_owned() };

        let x = 0.5*960.0-0.5*text_width;
        let mut y = 0.5*540.0-0.5*self.font_size;

        self.set_font_size(2.0*old_font_size);
        self.print_on_canvas(dt, &content, x, y, text_width, text_col);
        self.set_font_size(old_font_size);

        content = if state.steal_success || state.score_success { "Succes".to_owned() } else { "Failed".to_owned() };
        y += 3.0*self.font_size;
        self.print_on_canvas(dt, &content, x, y, text_width, text_col);
    }

    fn draw_player_ratings(&self, dt:&mut Pixmap, state:&mut State)
    {
        let positions = Display::get_circle_positions(0.225);
        let max_width = 3.0*self.card_width;
        let text_col = Color::from_rgba8(0x44,0x44,0x44,0xFF);

        for i in 0..state.hands.len()
        {
            let x = positions[i].x - 0.5*max_width;
            let y = positions[i].y - 0.5*self.font_size;
            let mut content = state.last_player_ratings[i].to_string();
            if content == "-100" { content = "X".to_owned(); }

            self.print_on_canvas(dt, &content, x, y, max_width, text_col);
        }
    }

    fn draw_top_card(&self, dt:&mut Pixmap, state:&mut State)
    {
        // draw top card in center of table
        let top_card = state.deck.last().unwrap();
        let values = vec![top_card.back2, top_card.back1, top_card.color];
        let card_width = 2.0*self.card_width;
        let card_height = 2.0*self.card_height;
        let center_x = 0.5*960.0 - 0.5*card_width;
        let center_y = 0.5*540.0 - 0.5*card_height;

        let mut stroke = Stroke::default();
        stroke.width = 0.5*self.margin;
        stroke.line_cap = LineCap::Round;

        let mut paint_stroke = Paint::default();
        paint_stroke.set_color(Color::BLACK);
        paint_stroke.anti_alias = true;

        for (k,v) in values.iter().enumerate()
        {
            let temp_height = (3.0 - k as f32) / 3.0 * card_height;
            let rect = Rect::from_xywh(center_x, center_y, card_width, temp_height).unwrap();
            let path = PathBuilder::from_rect(rect);

            let mut paint = Paint::default();
            paint.anti_alias = true;
            paint.set_color(Display::get_card_color(*v));

            dt.fill_path(
                &path, 
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None
            );
            
            let is_true_color:bool = k == values.len() - 1;
            if is_true_color
            {
                dt.stroke_path(
                    &path, 
                    &paint_stroke, 
                    &stroke, 
                    Transform::identity(),
                    None
                );
            }

        }
    }

    fn new_pixmap(&self) -> Pixmap
    {
        let mut dt = Pixmap::new(960, 540).unwrap();

        // solid slightly off-white background
        dt.fill(Color::from_rgba8(0xFA, 0xFA, 0xFA, 0xFF));
        return dt;
    }

    fn save_png(&self, dt:Pixmap, state:&mut State)
    {
        let turn_num = state.screenshot_num;
        state.screenshot_num += 1;
        
        let filename = "turn_".to_owned() + &format!("{:0>4}", turn_num.to_string());
        let filepath = "images_gamestate/".to_owned() + &filename + ".png";
        let result = dt.save_png(filepath);
        match result {
            Ok(_res) => println!("Saved image!"),
            Err(_err) => println!("Error saving image"),
        };
    }

    fn get_circle_positions(s:f32) -> Vec<Pos>
    {
        return vec![
            Pos { x: 0.5*960.0, y: (0.5-s)*540.0 },
            Pos { x: (0.5+s)*960.0, y: 0.5*540.0 },
            Pos { x: 0.5*960.0, y: (0.5+s)*540.0 },
            Pos { x: (0.5-s)*960.0, y: 0.5*540.0 }
        ];
    }

    fn draw_hand(&mut self, dt:&mut Pixmap, num:usize, state:&State, turn_done: bool)
    {
        if state.hands[num].len() <= 0 { return; }

        let positions = Display::get_circle_positions(0.35);
        let mut base_pos = positions[num].clone();

        // center the hand
        let num_cols = (state.hands[num].len() + 2) as f32;
        base_pos.x -= 0.5*num_cols*(self.card_width+self.margin);
        base_pos.y -= 0.5*(self.card_height+self.margin);

        // basic properties for stroking; same for all cards (white thick stroke)
        let mut stroke = Stroke::default();
        stroke.width = 0.5*self.margin;
        stroke.line_cap = LineCap::Round;

        let mut paint_stroke = Paint::default();
        paint_stroke.set_color(Color::WHITE);
        paint_stroke.anti_alias = true;

        let its_our_turn = state.cur_player == num;
        if its_our_turn
        {
            stroke.width = 0.5*self.margin;
            paint_stroke.set_color(Color::from_rgba8(0xFF, 0x00, 0x00, 0xFF));
        }

        let we_are_victim = state.last_victim == num;
        if we_are_victim && turn_done
        {
            stroke.width = 0.25*self.margin;
            paint_stroke.set_color(Color::BLACK);
        }

        // display the score
        let pos_above = base_pos.y - self.font_size;
        let old_font_size = self.font_size;
        let score_text_width = 2.0*self.card_width;
        let text_col = Color::from_rgba8(0,0,0,0xFF);
        self.set_font_size(2.0*old_font_size);
        self.print_on_canvas(dt, &state.score[num].to_string(), base_pos.x, base_pos.y, score_text_width, text_col);
        self.set_font_size(old_font_size);

        // for each card, draw a rectangle in the right color (offset to show all cards nicely)
        // NOTE: this starts at column 2, of course, to make room for the score
        let mut col:f32 = 1.0;
        for (k,v) in state.hands[num].iter()
        {
            col += 1.0;
            let x = base_pos.x + col*(self.card_width+self.margin);
            
            let mut paint = Paint::default();
            paint.set_color(Display::get_card_color(*k));

            for i in 0..(*v)
            {
                let row = i as f32;
                let y = base_pos.y + row*(0.2*self.card_height);

                let rect = Rect::from_xywh(x, y, self.card_width, self.card_height).unwrap();
                let path = PathBuilder::from_rect(rect);
    
                dt.fill_path(
                    &path, 
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    None
                );

                dt.stroke_path(
                    &path, 
                    &paint_stroke, 
                    &stroke, 
                    Transform::identity(),
                    None
                );
            }
        }
    }

    // Orange, yellow, green, pink, blue, purple, red
    pub fn get_card_color(col:u8) -> Color
    {
        let cols = vec![
            Color::from_rgba8(0xFF, 0xA5, 0x00, 0xFF),
            Color::from_rgba8(0xFF, 0xFF, 0x00, 0xFF),
            Color::from_rgba8(0x00, 0xFF, 0x00, 0xFF),
            Color::from_rgba8(0xFF, 0xA5, 0xCC, 0xFF),
            Color::from_rgba8(0x00, 0x00, 0xFF, 0xFF),
            Color::from_rgba8(0x00, 0xFF, 0xFF, 0xFF),
            Color::from_rgba8(0xFF, 0x00, 0x00, 0xFF)
        ];
        return cols[col as usize];
    }

    
}