use crate::frame::Frame;
use crate::game::Game;
use crate::scores::Scores;
use crate::screen::Screen;
use crate::settings::Settings;

mod scores;
mod settings;
mod sound;
mod globals;
mod sprite;
mod screen;
mod resources;
mod sprites;
mod game;
mod frame;
mod animation;
mod entity;

fn main() {
    let settings = Settings::new();
    let mut scores = Scores::new();
    // let sound = Sound::default().init();
    let mut frame = Frame::new();

    let sdl = sdl2::init();
    let sdl = match sdl {
        Err(err) => { panic!("Can't initialize SDL {}", err) }
        Ok(sdl) => sdl
    };

    let mut screen = Screen::new(&sdl);

    scores.load(&settings);

    let mut game = Game::new(&sdl, &settings, &scores, &mut frame);
    game.start(&mut screen);
}

