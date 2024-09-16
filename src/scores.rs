use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::str;
use crate::settings::Settings;

const SCORES_HIGH_LINES: usize = 12;
const SCORES_LINES: usize = SCORES_HIGH_LINES - 1;


#[derive(Debug)]
pub struct Scores {
    pub level_flag: bool,
    pub initial_path: String,
    pub saved_path: String,
    pub scores: String,
    pub high: String,
    pub scores_high: [i64; SCORES_HIGH_LINES],
    pub scores_init: [String; SCORES_LINES],
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
}

// void loadscores(void);
// void showtable(void);
// void zeroscores(void);
// void writecurscore(int col);
// void drawscores(void);
// void initscores(void);
// void endofgame(void);
// void scorekill(int n);
// void scorekill2(void);
// void scoreemerald(int n);
// void scoreoctave(int n);
// void scoregold(int n);
// void scorebonus(int n);
// void scoreeatm(int n,int msc);
// void addscore(int n,Sint4 score);
//
// #ifdef INTDRF
// Sint5 getscore0(void);
// #endif
//
// extern Uint4 bonusscore;
// extern Sint5 scoret;
//
// extern char scoreinit[11][4];

