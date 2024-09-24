pub struct Bag {
    pub x: i32,
    pub y: i32,
    pub h: i32,
    pub v: i32,
    pub xr: i32,
    pub yr: i32,
    pub dir: i32,
    pub wt: i32,
    pub gt: i32,
    pub fallh: i32,
    pub wobbling: bool,
    pub unfallen: bool,
    pub exist: bool,
}

impl Bag {
    pub fn new(x: i32, y: i32) -> Bag {
        Bag {
            exist: true,
            gt: 0,
            fallh: 0,
            dir: -1,
            wobbling: false,
            wt: 15,
            unfallen: true,
            x: x * 20 + 12,
            y: y * 18 + 18,
            h: x,
            v: y,
            xr: 0,
            yr: 0,
        }
    }
}