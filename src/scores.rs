use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use crate::screen::Screen;
use crate::settings::Settings;

const SCORES_HIGH_LINES: usize = 12;
const SCORES_LINES: usize = SCORES_HIGH_LINES - 1;
const MAX_SCORES: usize = 999999;
const BONUS_SCORES: usize = 20000;

#[derive(Debug)]
pub struct Scores {
    pub level_flag: bool,
    pub initial_path: String,
    pub saved_path: String,
    pub scores: String,
    pub high: String,
    pub scores_high: [i64; SCORES_HIGH_LINES],
    pub scores_init: [String; SCORES_LINES],
    pub current_scores: usize,
    pub next_bonus: usize,
    pub penalty: usize,
}

impl Scores {
    pub fn new() -> Scores {
        Scores {
            level_flag: false,
            initial_path: String::from("DIGGER.SCO"),
            saved_path: String::new(),
            scores: String::new(),
            high: String::new(),
            scores_high: [0; SCORES_HIGH_LINES],
            scores_init: [const { String::new() }; SCORES_LINES],
            current_scores: 0,
            next_bonus: BONUS_SCORES,
            penalty: 0
        }
    }

    pub fn load(&mut self, settings: &Settings) {
        self.read_scores();
        let mut p: usize = 0;
        if settings.is_gauntlet_mode() {
            p = 111;
        }
        if settings.get_diggers() == 2 {
            p += 222;
        }

        if self.scores.len() == 0 {
            for i in 0..SCORES_LINES {
                self.scores_high[i + 1] = 0;
                self.scores_init[i] = String::from("...");
            }
        } else {
            p += 1;
            for i in 1..SCORES_LINES {
                    let s = &self.scores[p..p + 3];
                self.scores_init[i] = String::from(s);
                p += 3;
                let s = &self.scores[p..p + 8];
                self.high = String::from(s);
                self.scores_high[i + 1] = self.high.trim().parse().unwrap();
                p+=8
            }
        }
    }

    fn read_scores(&mut self) {
        if self.level_flag {
            let file = File::open(&self.saved_path);

            if let Ok(file) = file {
                let mut buf = BufReader::new(file);
                buf.seek(SeekFrom::Start(1202)).unwrap();
                buf.read_to_string(&mut self.scores).unwrap();
            }

            return;
        }

        let file = File::open(&self.initial_path);
        if let Ok(file) = file {
            let mut buf = BufReader::new(file);
            buf.read_to_string(&mut self.scores).unwrap();
        }
    }

    pub fn add_scores(&mut self, scores: usize) {
        self.current_scores += scores;
        if self.current_scores > MAX_SCORES {
            self.current_scores = 0;
        }

        self.penalty += 3;
    }

    pub fn is_new_life(&self) -> bool {
        self.current_scores >= self.next_bonus
    }

    pub fn increase_bonus_score(&mut self) {
        self.next_bonus += BONUS_SCORES;
    }

    pub fn clean_penalty(&mut self) {
        self.penalty = 0;
    }

    pub fn draw_saved_scores(&self, screen: &mut Screen) {
        screen.draw_text("HIGH SCORES", 16, 25, 3);
        let mut color = 2;
        for i in 1..self.scores_init.len() {
            let mut line: String = String::from(&self.scores_init[i]);
            let score: String = format!("{:6}", self.scores_high[i + 1]);
            line = String::from(line) + "  " + &score;
            screen.draw_text(&line, 16, (31 + 13 * i) as i32, color);
            color = 1;
        }
    }

    pub fn draw_current_scores(&self, screen: &mut Screen) {
        screen.draw_number(self.current_scores, 0, 0, 6, 1);
    }
}
