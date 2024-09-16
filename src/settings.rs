pub struct Settings {
    gauntlet_mode: bool,
    diggers: usize,
    players: usize,
    escape: bool
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            gauntlet_mode: false,
            diggers: 1,
            players: 1,
            escape: false
        }
    }

    pub fn is_gauntlet_mode(&self) -> bool {
        self.gauntlet_mode
    }

    pub fn set_gauntlet_mode(&mut self, mode: bool) {
        self.gauntlet_mode = mode;
    }

    pub fn get_diggers(&self) -> usize {
        self.diggers
    }

    pub fn is_escape(&self) -> bool {
        self.escape
    }

    pub fn set_escape(&mut self, flag: bool) {
        self.escape = flag;
    }

    pub fn set_players(&mut self, n: usize) {
        self.players = n;
    }

    pub fn get_players(&self) -> usize {
        self.players
    }
}
