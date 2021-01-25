/// Contains functions defining the game and valid moves.
use peroxide::prelude::*;

/// Explicit player value
#[derive(Debug)]
pub enum Player {
    Max,
    Min,
}

impl Player {
    pub fn next_player(&self) -> Player {
        match self {
            Player::Max => Player::Min,
            Player::Min => Player::Max,
        }
    }
}

/// Store current player and board state
#[derive(Debug)]
pub struct GameState {
    pub player: Player,
    pub board: Matrix,
}

impl GameState {
    pub fn advance(&self, src: Location) -> GameState {
        let new_m;
        match self.player {
            Player::Max => new_m = src.m + 1,
            Player::Min => new_m = src.m - 1,
        }

        let dst = Location { m: new_m, n: src.n };
        self.update(dst, src)
    }

    pub fn update(&self, dst: Location, src: Location) -> GameState {
        let p = self.player.next_player();
        let mut b = self.board.clone();

        println!("src: {:?}, val: {:?}", src, self.board[(src.m, src.n)]);
        println!("dst: {:?}, val: {:?}", dst, self.board[(dst.m, dst.n)]);
        b[(dst.m, dst.n)] = self.board[(src.m, src.n)];
        b[(src.m, src.n)] = 0.0;

        GameState {
            player: p,
            board: b,
        }
    }
}

/// m by n index values within the game matrix
#[derive(Debug)]
pub struct Location {
    pub m: usize,
    pub n: usize,
}

impl Location {
    pub fn check_advance(&self, s: &GameState) -> bool {
        let dst;
        match s.player {
            Player::Max => {
                dst = Location {
                    m: self.m + 1,
                    n: self.n,
                }
            }
            Player::Min => {
                dst = Location {
                    m: self.m - 1,
                    n: self.n,
                }
            }
        }

        dst.is_in_bounds() && s.board[(dst.m, dst.n)] == 0.0
    }

    pub fn is_in_bounds(&self) -> bool {
        (0..3).contains(&self.m) && (0..3).contains(&self.n)
    }
}

/// Initialize new game state
pub fn new_game() -> GameState {
    let p = Player::Max;
    let b = matrix(c!(1, 1, 1, 0, 0, 0, -1, -1, -1), 3, 3, Row);
    GameState {
        player: p,
        board: b,
    }
}
