use std::io::Read;
use flate2::read::{GzDecoder, ZlibDecoder};
use crate::drawing::Drawing;
use crate::scores::Scores;
use crate::screen::Screen;
use crate::settings::Settings;
use crate::sound::Sound;
use crate::sprite::Sprite;

mod scores;
mod settings;
mod sound;
mod globals;
mod sprite;
mod drawing;
mod screen;
mod resources;

fn main() {
    let settings = Settings::new();
    let mut scores = Scores::new();
    let sound = Sound::default().init();
    let mut sprite = Sprite::default();
    let drawing = Drawing::new(&mut sprite);
    let mut screen = Screen::new();

    scores.load(&settings);


    // screen.show_t(&buf);
    // title.read_exact(&mut buf).unwrap();
    // println!("{:?}", buf);



    loop {
        screen.clean();
        screen.show_background();
        screen.show_game_name();
        screen.show_players(&settings);
        screen.show_scores(&scores);



        screen.render();
    //
    //     // println!("{drawing:#?}");
    //     // if settings.is_escape() {
    //     //     break;
    //     // }
    }
}

