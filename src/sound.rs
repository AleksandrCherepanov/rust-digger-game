use crate::globals::{FIREBALLS};

#[derive(Default)]
pub struct Sound {
    volume: usize,
    fall_flag: bool,
    fall_f: bool,
    fall_value: i16,
    fall_n: i16,
    wobble_flag: bool,
    wobble_n: i16,
    fire_flag: [bool; 2],
    explode_flag: [bool; 2],
    fire_n: [u16; 2],
    music_playing: bool,
    music_p: i16,
    bonus_flag: bool,
    bonus_n: i16,
    break_flag: bool,
    em_flag: bool,
    emerald_flag: bool,
    gold_flag: bool,
    eat_flag: bool,
    die_flag: bool,
    up_flag: bool,
}

impl Sound {
    pub fn init(&mut self) {
        self.volume = 1;
    }

    pub fn stop(&mut self) {
        self.fall_off();
        self.wobble_off();
        self.fire_off();
        self.music_off();
        self.bonus_off();
        self.explode_off();
        self.break_off();
        self.em_off();
        self.emerald_off();
        self.gold_off();
        self.eat_off();
        self.die_off();
        self.up_off();
    }

    fn fall_off(&mut self) {
        self.fall_flag = false;
        self.fall_n = 0;
    }

    fn wobble_off(&mut self) {
        self.wobble_flag = false;
        self.wobble_n = 0;
    }

    fn fire_off(&mut self) {
        for i in 0..FIREBALLS {
            self.fire_flag[i] = false;
            self.fire_n[i] = 0;
        }
    }

    fn explode_off(&mut self) {
        for i in 0..FIREBALLS {
            self.explode_flag[i] = false;
        }
    }

    fn music_off(&mut self) {
        self.music_playing = false;
        self.music_p = 0;
    }

    fn bonus_off(&mut self) {
        self.bonus_flag = false;
        self.bonus_n = 0;
    }

    fn break_off(&mut self) {
        self.break_flag = false
    }

    fn em_off(&mut self) {
        self.em_flag = false
    }

    fn emerald_off(&mut self) {
        self.emerald_flag = false;
    }
    fn gold_off(&mut self) {
        self.gold_flag = false;
    }
    fn eat_off(&mut self) {
        self.eat_flag = false;
    }
    fn die_off(&mut self) {
        self.die_flag = false;
    }
    fn up_off(&mut self) {
        self.up_flag = false;
    }
}
