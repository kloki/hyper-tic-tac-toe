#[derive(Clone, Copy, PartialEq)]
pub enum Pos {
    LU,
    MU,
    RU,
    LM,
    MM,
    RM,
    LD,
    MD,
    RD,
}

impl Pos {
    pub fn class(&self) -> &str {
        match self {
            Pos::LU => "lu",
            Pos::MU => "mu",
            Pos::RU => "ru",
            Pos::LM => "lm",
            Pos::MM => "mm",
            Pos::RM => "rm",
            Pos::LD => "ld",
            Pos::MD => "md",
            Pos::RD => "rd",
        }
    }
    pub fn index(&self) -> usize {
        match self {
            Pos::LU => 0,
            Pos::MU => 1,
            Pos::RU => 2,
            Pos::LM => 3,
            Pos::MM => 4,
            Pos::RM => 5,
            Pos::LD => 6,
            Pos::MD => 7,
            Pos::RD => 8,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    Open,
    A,
    B,
}

impl Player {
    pub fn class(&self) -> &str {
        match self {
            Player::A => "player_a",
            Player::B => "player_b",
            Player::Open => "player_none",
        }
    }
    pub fn value(&self) -> &str {
        match self {
            Player::A => "X",
            Player::B => "O",
            Player::Open => " ",
        }
    }
    pub fn resolved(&self) -> bool {
        *self != Player::Open
    }
}

#[derive(Copy, Clone)]
pub struct SubBoard {
    tiles: [Player; 9],
    pub winner: Player,
}

impl SubBoard {
    pub fn new() -> Self {
        SubBoard {
            tiles: [Player::Open; 9],
            winner: Player::Open,
        }
    }

    pub fn full(&self) -> bool {
        self.tiles
            .iter()
            .filter(|x| !x.resolved())
            .collect::<Vec<_>>()
            .len()
            == 0
    }

    pub fn update_state(&mut self) -> bool {
        for i in 0..3 {
            let index = i * 3;
            //horizontal
            if self.tiles[index].resolved()
                && self.tiles[index + 1] == self.tiles[index]
                && self.tiles[index] == self.tiles[index + 2]
            {
                self.winner = self.tiles[index];
                return true;
            }
            //vertial
            let index = i;
            if self.tiles[index].resolved()
                && self.tiles[index + 3] == self.tiles[index]
                && self.tiles[index] == self.tiles[index + 6]
            {
                self.winner = self.tiles[index];
                return true;
            }
        }
        if self.tiles[0].resolved()
            && self.tiles[4] == self.tiles[0]
            && self.tiles[8] == self.tiles[0]
        {
            self.winner = self.tiles[0];
            return true;
        }
        if self.tiles[2].resolved()
            && self.tiles[4] == self.tiles[2]
            && self.tiles[6] == self.tiles[2]
        {
            self.winner = self.tiles[2];
            return true;
        }
        false
    }

    pub fn set(&mut self, pos: Pos, player: Player) -> bool {
        self.tiles[pos.index()] = player;
        self.update_state()
    }
}

pub struct Board {
    pub boards: [SubBoard; 9],
    pub winner: Player,
    pub current: Player,
    last_move: Option<Pos>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            boards: [SubBoard::new(); 9],
            winner: Player::Open,
            current: Player::A,
            last_move: None,
        }
    }

    pub fn update_state(&mut self) -> bool {
        for i in 0..3 {
            let index = i * 3;
            //horizontal
            if self.boards[index].winner.resolved()
                && self.boards[index + 1].winner == self.boards[index].winner
                && self.boards[index].winner == self.boards[index + 2].winner
            {
                self.winner = self.boards[index].winner;
                return true;
            }
            //vertial
            let index = i;
            if self.boards[index].winner.resolved()
                && self.boards[index + 3].winner == self.boards[index].winner
                && self.boards[index].winner == self.boards[index + 6].winner
            {
                self.winner = self.boards[index].winner;
                return true;
            }
        }
        if self.boards[0].winner.resolved()
            && self.boards[4].winner == self.boards[0].winner
            && self.boards[8].winner == self.boards[0].winner
        {
            self.winner = self.boards[0].winner;
            return true;
        }
        if self.boards[2].winner.resolved()
            && self.boards[4].winner == self.boards[2].winner
            && self.boards[6].winner == self.boards[2].winner
        {
            self.winner = self.boards[2].winner;
            return true;
        }
        false
    }
    pub fn set(&mut self, pos1: Pos, pos2: Pos) {
        let resolved = self.boards[pos1.index()].set(pos2.clone(), self.current);

        if resolved {
            self.update_state();
            self.last_move = None;
        } else {
            if self.boards[pos2.index()].winner.resolved() {
                self.last_move = None;
            } else {
                self.last_move = Some(pos2);
            }
        }
        if self.current == Player::A {
            self.current = Player::B;
        } else {
            self.current = Player::A;
        }
    }

    pub fn get(&self, pos1: Pos, pos2: Pos) -> Player {
        self.boards[pos1.index()].tiles[pos2.index()]
    }
    pub fn clickable(&self, pos1: Pos, pos2: Pos) -> bool {
        if self.winner.resolved() {
            return false;
        }

        if self.boards[pos1.index()].tiles[pos2.index()].resolved() {
            return false;
        }
        match self.last_move {
            None => true,
            Some(s) if s == pos1 => true,
            _ => false,
        }
    }
    pub fn clickable_board(&self, pos: Pos) -> bool {
        if self.winner.resolved() {
            return false;
        }
        if self.boards[pos.index()].full() {
            return false;
        }

        match self.last_move {
            None => true,
            Some(s) if s == pos => true,
            _ => false,
        }
    }
    pub fn resolved(&self, pos: Pos) -> bool {
        self.boards[pos.index()].winner.resolved()
    }

    pub fn tied(&self) -> bool {
        self.boards
            .iter()
            .filter(|x| !x.full())
            .collect::<Vec<_>>()
            .len()
            == 0
    }
}
