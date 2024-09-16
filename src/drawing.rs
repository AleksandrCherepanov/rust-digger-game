use crate::globals::{BAGS, BONUSES, DIGGERS, FIREBALLS, FIRST_BONUS, FIRST_DIGGER, FIRST_FIRE_BALL, FIRST_MONSTER, LAST_BONUS, LAST_DIGGER, LAST_FIRE_BALL, MONSTERS};
use crate::sprite::Sprite;

#[derive(Debug)]
pub struct Drawing<'a> {
    sprite: &'a mut Sprite,
    monbufs: [String; MONSTERS],
    bagbufs: [String; BAGS],
    bonusbufs: [String; BONUSES],
    diggerbufs: [String; DIGGERS],
    firebufs: [String; FIREBALLS],

    mon: [i16; MONSTERS],
    monspd: [i16; MONSTERS],
    dig: [i16; DIGGERS],
    digspd: [i16; DIGGERS],
    fire: [i16; FIREBALLS]
}

impl<'a> Drawing<'a> {
    pub fn new(sprite: &mut Sprite) -> Drawing {
        Drawing {
            sprite,
            monbufs: [const { String::new() }; MONSTERS],
            bagbufs: [const { String::new() }; BAGS],
            bonusbufs: [const { String::new() }; BONUSES],
            diggerbufs: [const { String::new() }; DIGGERS],
            firebufs: [const { String::new() }; FIREBALLS],
            mon: [0; MONSTERS],
            monspd: [0; MONSTERS],
            dig: [0; DIGGERS],
            digspd: [0; DIGGERS],
            fire: [0; FIREBALLS]
        }
    }

    pub fn create_mb(&mut self) {
        for i in 0..BAGS {
            self.sprite.create(FIREBALLS + i, 62, &self.bagbufs[i], 4, 15, 0, 0);
        }
        for i in 0..MONSTERS {
            self.sprite.create(FIRST_MONSTER + i, 71, &self.monbufs[i], 4, 15, 0, 0);
        }

        self.create_dbf();

        for i in 0..MONSTERS {
            self.mon[i] = 0;
            self.monspd[i] = 1;
        }
    }

    fn create_dbf(&mut self) {
        for i in 0..DIGGERS {
            self.digspd[i] = 1;
            self.dig[i] = 0;
        }

        for i in 0..FIREBALLS {
            self.fire[i] = 0;
        }

        for i in FIRST_DIGGER..LAST_DIGGER {
            self.sprite.create(i, 0, &self.diggerbufs[i-FIRST_DIGGER], 4, 15, 0, 0);
        }

        for i in FIRST_BONUS..LAST_BONUS {
            self.sprite.create(i, 81, &self.bonusbufs[i-FIRST_BONUS], 4, 15, 0, 0);
        }

        for i in FIRST_FIRE_BALL..LAST_FIRE_BALL {
            self.sprite.create(i, 82, &self.firebufs[i-FIRST_FIRE_BALL], 2, 8, 0, 0);
        }
    }
}


