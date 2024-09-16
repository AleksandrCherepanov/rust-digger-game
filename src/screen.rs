use sdl2::EventPump;
use sdl2::rect::Point;
use sdl2::render::{WindowCanvas};
use crate::resources::background::draw_background;
use crate::resources::palette::get_color;
use crate::resources::text;
use crate::scores::Scores;
use crate::settings::Settings;

pub struct Screen {
    canvas: WindowCanvas,
    eventPump: EventPump,
}

impl Screen {
    pub fn new() -> Screen {
        let sdl = sdl2::init();
        let sdl = match sdl {
            Err(err) => { panic!("Can't initialize SDL {}", err) }
            Ok(sdl) => sdl
        };

        let video = sdl.video().unwrap();
        let window = video.window("D I G G E R", 640, 400).build().unwrap();
        let canvas = window.into_canvas().build().unwrap();
        sdl.mouse().show_cursor(false);

        Screen {
            canvas,
            eventPump: sdl.event_pump().unwrap(),
        }
    }

    pub fn show_game_name(&mut self) {
        text::print(&mut self.canvas, "D I G G E R", 100, 0, 3);
    }

    pub fn show_players(&mut self, settings: &Settings) {
        if settings.get_diggers() == 2 {
            if settings.is_gauntlet_mode() {
                text::print(&mut self.canvas, "TWO PLAYER", 180, 25, 3);
                text::print(&mut self.canvas, "GAUNTLET", 192, 39, 3);
                return;
            }

            text::print(&mut self.canvas, "TWO PLAYER", 180, 25, 3);
            text::print(&mut self.canvas, "SIMULTANEOUS", 170, 39, 3);
            return;
        }

        if settings.is_gauntlet_mode() {
            text::print(&mut self.canvas, "GAUNTLET", 192, 25, 3);
            text::print(&mut self.canvas, "MODE", 216, 39, 3);
            return;
        }

        if settings.get_players() == 1 {
            text::print(&mut self.canvas, "ONE", 220, 25, 3);
            text::print(&mut self.canvas, " PLAYER ", 192, 39, 3);
            return;
        }

        text::print(&mut self.canvas, "TWO", 220, 25, 3);
        text::print(&mut self.canvas, " PLAYERS", 184, 39, 3);
    }

    pub fn show_scores(&mut self, scores: &Scores) {
        text::print(&mut self.canvas, "HIGH SCORES", 16, 25, 3);
        let mut color = 2;
        for i in 1..scores.scores_init.len() {
            let mut line: String = String::from(&scores.scores_init[i]);
            let score: String = format!("{:6}", scores.scores_high[i + 1]);
            line = String::from(line) + "  " + &score;
            text::print(&mut self.canvas, &line, 16, (31 + 13*i) as i32, color);
            color = 1;
        }
    }

    pub fn show_background(&mut self) {
        draw_background(&mut self.canvas);
    }

    pub fn clean(&mut self) {
        self.canvas.clear();
    }

    pub fn render(&mut self) {
        self.canvas.present();
    }
}
