/// Contains functions defining the game and valid moves.
use ndarray::prelude::*;
use std::fmt;

/// Explicit player value
pub enum Player {
    Max,
    Min,
}

impl Player {
    pub fn next(&self) -> Player {
        match *self {
            Player::Max => Player::Min,
            Player::Min => Player::Max,
        }
    }

    pub fn value(&self) -> i8 {
        match *self {
            Player::Max => 1,
            Player::Min => -1,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::Max => write!(f, "Max"),
            Player::Min => write!(f, "Min"),
        }
    }
}

/// Store current player and board state
pub struct GameState {
    pub player: Player,
    pub board: Array<i8, Ix2>,
}

impl GameState {
    pub fn actions(&self) {
        for (idx, val) in self.board.indexed_iter() {
            if *val == self.player.value() {
                let src = Location { m: idx.0, n: idx.1 };

                if src.check_advance(&self) {
                    println!("({},{}) could advance.", idx.0, idx.1);
                }

                if src.check_capture_left(&self) {
                    println!("({},{}) could capture left.", idx.0, idx.1);
                }

                if src.check_capture_right(&self) {
                    println!("({},{}) could capture right.", idx.0, idx.1);
                }
            }
            //println!("idx: {},{}, val: {}", idx.0, idx.1, val);
        }
    }

    pub fn advance(&self, src: Location) -> GameState {
        let new_m;
        match self.player {
            Player::Max => new_m = src.m + 1,
            Player::Min => new_m = src.m - 1,
        }

        let dst = Location { m: new_m, n: src.n };
        self.update(dst, src)
    }

    pub fn capture_left(&self, src: Location) -> GameState {
        let new_m;
        let new_n;
        match self.player {
            Player::Max => {
                new_m = src.m + 1;
                new_n = src.n - 1;
            }
            Player::Min => {
                new_m = src.m - 1;
                new_n = src.n - 1;
            }
        }

        let dst = Location { m: new_m, n: new_n };
        self.update(dst, src)
    }

    pub fn capture_right(&self, src: Location) -> GameState {
        let new_m;
        let new_n;
        match self.player {
            Player::Max => {
                new_m = src.m + 1;
                new_n = src.n + 1;
            }
            Player::Min => {
                new_m = src.m - 1;
                new_n = src.n + 1;
            }
        }

        let dst = Location { m: new_m, n: new_n };
        self.update(dst, src)
    }

    pub fn update(&self, dst: Location, src: Location) -> GameState {
        let p = self.player.next();
        let mut b = self.board.clone();

        //println!("src: {:?}, val: {:?}", src, self.board[(src.m, src.n)]);
        //println!("dst: {:?}, val: {:?}", dst, self.board[(dst.m, dst.n)]);
        b[(dst.m, dst.n)] = self.board[(src.m, src.n)];
        b[(src.m, src.n)] = 0;

        GameState {
            player: p,
            board: b,
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\nBoard:\n{:4}\n{:4}\n{:4}\nNext Player: {}\n",
            self.board.row(0),
            self.board.row(1),
            self.board.row(2),
            self.player
        )
    }
}

/// m by n index values within the game matrix
pub struct Location {
    pub m: usize,
    pub n: usize,
}

impl Location {
    pub fn check_advance(&self, s: &GameState) -> bool {
        match s.player {
            Player::Max => {
                if self.m == 2 {
                    false
                } else {
                    s.board[(self.m + 1, self.n)] == 0
                }
            }
            Player::Min => {
                if self.m == 0 {
                    false
                } else {
                    s.board[(self.m - 1, self.n)] == 0
                }
            }
        }
    }

    pub fn check_capture_left(&self, s: &GameState) -> bool {
        match s.player {
            Player::Max => {
                if self.m == 2 || self.n == 0 {
                    false
                } else {
                    s.board[(self.m + 1, self.n - 1)] == Player::Min.value()
                }
            }
            Player::Min => {
                if self.m == 0 || self.n == 0 {
                    false
                } else {
                    s.board[(self.m - 1, self.n - 1)] == Player::Max.value()
                }
            }
        }
    }

    pub fn check_capture_right(&self, s: &GameState) -> bool {
        match s.player {
            Player::Max => {
                if self.m == 2 || self.n == 2 {
                    false
                } else {
                    s.board[(self.m + 1, self.n + 1)] == Player::Min.value()
                }
            }
            Player::Min => {
                if self.m == 0 || self.n == 2 {
                    false
                } else {
                    s.board[(self.m - 1, self.n + 1)] == Player::Max.value()
                }
            }
        }
    }

    pub fn is_in_bounds(&self) -> bool {
        (0..3).contains(&self.m) && (0..3).contains(&self.n)
    }
}

/// Initialize new game state
pub fn new_game() -> GameState {
    let p = Player::Max;
    let b = arr2(&[[1, 1, 1], [0, 0, 0], [-1, -1, -1]]);
    GameState {
        player: p,
        board: b,
    }
}
