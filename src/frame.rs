pub const TICKS_PER_FRAME: isize = 80000;

pub struct Frame {
    prev_ticks: isize,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            prev_ticks: 0
        }
    }

    pub fn tick(&mut self, sdl: &sdl2::Sdl) -> usize {
        let mut diff: isize = 0;

        if self.prev_ticks == 0 {
            self.prev_ticks = sdl.timer().unwrap().ticks() as isize;
        } else {
            diff = TICKS_PER_FRAME / 1000 - (sdl.timer().unwrap().ticks() as isize - self.prev_ticks);
            if diff > 0 {
                sdl.timer().unwrap().delay(diff as u32);
            }
            self.prev_ticks = sdl.timer().unwrap().ticks() as isize;
        }

        0
    }
}