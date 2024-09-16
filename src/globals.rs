pub const DIGGERS: usize = 2;
pub const FIREBALLS: usize = 2;
pub const BONUSES: usize = 1;
pub const BAGS: usize = 7;
pub const MONSTERS: usize = 6;
pub const SPRITES: usize = BONUSES + BAGS + MONSTERS + FIREBALLS + DIGGERS;

pub const FIRST_BONUS: usize = 0;
pub const LAST_BONUS: usize = FIRST_BONUS + BONUSES;
pub const FIRST_BAG: usize = LAST_BONUS;
pub const LAST_BAG: usize = FIRST_BAG + BAGS;
pub const FIRST_MONSTER: usize = LAST_BAG;
pub const LAST_MONSTER: usize = FIRST_MONSTER + MONSTERS;
pub const FIRST_FIRE_BALL: usize = LAST_MONSTER;
pub const LAST_FIRE_BALL: usize = FIRST_FIRE_BALL + FIREBALLS;
pub const FIRST_DIGGER: usize = LAST_FIRE_BALL;
pub const LAST_DIGGER: usize = FIRST_DIGGER + DIGGERS;
