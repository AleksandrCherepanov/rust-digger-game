use crate::globals::SPRITES;

#[derive(Default, Debug)]
pub struct Sprite {
    nch: [i16; SPRITES],
    ch: [i16; SPRITES+1],
    mov: [String; SPRITES],
    nwid: [i16; SPRITES],
    wid: [i16; SPRITES+1],
    nhei: [i16; SPRITES],
    hei: [i16; SPRITES+1],
    nbwid: [i16; SPRITES],
    bwid: [i16; SPRITES],
    nbhei: [i16; SPRITES],
    bhei: [i16; SPRITES],
    enf: [bool; SPRITES],

    // bool sprrdrwf[SPRITES+1];
    // bool sprrecf[SPRITES+1];
    // Sint4 sprx[SPRITES+1];
    // Sint4 spry[SPRITES+1];

}

impl Sprite {
    pub fn create(&mut self, n: usize, ch: i16, mov: &str, wid: i16, hei: i16, bwid: i16, bhei: i16) {
        self.nch[n] = ch;
        self.ch[n] = ch;

        self.mov[n] = String::from(mov);

        self.nwid[n] = wid;
        self.wid[n] = wid;

        self.nhei[n] = hei;
        self.hei[n] = hei;

        self.nbwid[n]=bwid;
        self.bwid[n]=bwid;

        self.nbhei[n]=bhei;
        self.bhei[n]=bhei;

        self.enf[n]=false;
    }
}
