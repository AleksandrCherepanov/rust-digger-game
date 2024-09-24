#[derive(Default)]
pub struct Digger {
    x: i32,
    y: i32,
    h: i32,
    v: i32,
    rx: i32,
    ry: i32,
    mdir: i32,
    dir: i32,
    bagtime: i32,
    rechargetime: i32,
    fx: i32,
    fy: i32,
    fdir: i32,
    expsn: i32,
    deathstage: i32,
    deathbag: i32,
    deathani: i32,
    deathtime: i32,
    emoctime: i32,
    emn: i32,
    msc: i32,
    lives: i32,
    ivt: i32,
    notfiring: bool,
    alive: bool,
    firepressed: bool,
    dead: bool,
    levdone: bool,
    invin: bool
}


impl Digger {
}