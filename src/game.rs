use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl};
use crate::animation::Animation;
use crate::entity::level::Level;
use crate::frame::{Frame, TICKS_PER_FRAME};
use crate::scores::Scores;
use crate::screen::Screen;
use crate::settings::Settings;

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
            screen.initial();

            let should_continue = self.main_menu(screen, &mut event_listener);
            if !should_continue {
                break;
            }

            let mut level = Level::new(1);
            level.init_field();
            level.init_emeralds();
            level.init_bags();
            level.init_diggers();

            let mut digger_animation = Animation::new(level.digger.x, level.digger.y);
            loop {
                for event in event_listener.poll_iter() {
                    match event {
                        Event::Quit { .. } => break 'exit,
                        _ => {}
                    }
                }

                level.draw(screen);
                level.digger.animate(screen, &mut digger_animation, Animation::right());

                screen.draw_text("           ",100,0,3);
                self.scores.draw_current_scores(screen);
                level.digger.draw_lives(screen);

                self.new_frame(screen);
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

    fn main_menu(&mut self, screen: &mut Screen, event_listener: &mut EventPump) -> bool {
        self.scores.draw_saved_scores(screen);

        let mut nobbin_anim = Animation::new(292, 63);
        let mut hobbin_anim = Animation::new(292, 82);
        let mut digger_anim = Animation::new(292, 101);
        let mut x = 292;
        loop {
            for event in event_listener.poll_iter() {
                match event {
                    Event::Quit { .. } => return false,
                    Event::KeyDown { keycode:Some(Keycode::Return), .. } => self.started = true,
                    _ => {}
                }
            }
            if self.started {
                return true;
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
