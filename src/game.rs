use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use crate::animation::Animation;
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
            screen.initial(&self.settings, &self.scores);

            let mut nobbin_anim = Animation::new(292, 63);
            let mut hobbin_anim = Animation::new(292, 82);
            let mut digger_anim = Animation::new(292, 101);
            let mut x = 292;
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

                screen.clean_initial_items(self.current_frame);

                self.new_frame(screen);
                x = screen.play_initial_nobbin_animation(&mut nobbin_anim, self.current_frame, x);
                x = screen.play_initial_hobbin_animation(&mut hobbin_anim, self.current_frame, x);
                x = screen.play_initial_digger_animation(&mut digger_anim, self.current_frame, x);

                screen.show_initial_items(self.current_frame);

                if self.current_frame > 250 {
                    self.current_frame = 0;
                    nobbin_anim = Animation::new(292, 63);
                    hobbin_anim = Animation::new(292, 82);
                    digger_anim = Animation::new(292, 101);
                    x = 292;
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
}
