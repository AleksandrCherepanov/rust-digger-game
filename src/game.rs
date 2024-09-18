use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use crate::frame::{Frame, TICKS_PER_FRAME};
use crate::resources::text;
use crate::scores::Scores;
use crate::screen::Screen;
use crate::settings::Settings;
use crate::sprites::bag::Bag;
use crate::sprites::bonus::Bonus;
use crate::sprites::digger::Digger;
use crate::sprites::emerald::Emerald;
use crate::sprites::hobbin::Hobbin;
use crate::sprites::nobbin::Nobbin;

pub struct Game<'a> {
    sdl: &'a Sdl,
    settings: &'a Settings,
    scores: &'a Scores,
    frame: &'a mut Frame,
    current_time: usize,
    current_frame: usize,
    started: bool,
}

impl<'a> Game<'a> {
    pub fn new(
        sdl: &'a Sdl,
        settings: &'a Settings,
        scores: &'a Scores,
        frame: &'a mut Frame,
    ) -> Game<'a> {
        Game {
            sdl,
            settings,
            scores,
            frame,
            current_time: 0,
            current_frame: 0,
            started: false,
        }
    }

    pub fn start(&mut self, screen: &mut Screen) {
        let mut event_listener = self.sdl.event_pump().unwrap();
        screen.clean();
        'exit: loop {
            self.initial_screen(screen);

            loop {
                for event in event_listener.poll_iter() {
                    match event {
                        Event::Quit { .. } => break 'exit,
                        _ => {}
                    }
                }
                if self.started {
                    break;
                }

                if self.current_frame == 0 {
                    let mut i = 54;
                    while (i < 174) {
                        screen.draw_text("            ", 164, i, 0);
                        i+=12;
                    }
                }

                self.new_frame(screen);
                if (self.current_frame == 20) { // 50
                    screen.draw_sprite::<Nobbin>(0, 292, 63);
                }
                if (self.current_frame == 40) { // 90
                    screen.draw_sprite::<Hobbin>(0, 292, 82);
                }
                if (self.current_frame == 60) { // 130
                    screen.draw_sprite::<Digger>(1, 292, 101);
                }
                if (self.current_frame == 80) { // 178
                    screen.draw_sprite::<Bag>(0, 292, 120);
                }
                if (self.current_frame == 100) { // 198
                    screen.draw_sprite::<Emerald>(0, 292, 141);
                }
                if (self.current_frame == 120) { // 218
                    screen.draw_sprite::<Bonus>(0, 292, 158);
                }

                if self.current_frame > 250 {
                    self.current_frame = 0;
                }
            }
        }
    }

    pub fn new_frame(&mut self, screen: &mut Screen) {
        loop {
            screen.render();
            let time = self.frame.tick(self.sdl);
            if !(self.current_time + TICKS_PER_FRAME as usize > time && time > self.current_time) {
                break;
            }
            self.current_time = 0;
        }
        self.current_frame += 1;
    }

    fn initial_screen(&mut self, screen: &mut Screen) {
        screen.show_background();
        screen.show_game_name();
        screen.show_players(self.settings);
        screen.show_scores(self.scores);
    }
}
