/// Contains functions defining the game and valid actions.

use ndarray::prelude::*;
use std::fmt;

/// All possible actions
pub enum Action {
    Advance,
    CaptureLeft,
    CaptureRight,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::Advance => write!(f, "Advance"),
            Action::CaptureLeft => write!(f, "CaptureLeft"),
            Action::CaptureRight => write!(f, "CaptureRight"),
        }
    }
}

/// Hold all possible actions for a given turn
type ActionList = Vec<PlayerAction>;

/// Associate current player and board state
pub struct GameState {
    pub player: Player,
    pub board: Array<i8, Ix2>,
}

impl GameState {
    /// Return `ActionList`, a vector of all legal moves given the `Gamestate`
    pub fn actions(&self) -> ActionList {
        let mut action_list: Vec<PlayerAction> = Vec::new();
        for (idx, val) in self.board.indexed_iter() {
            if *val == self.player.value() {
                let src = Location { m: idx.0, n: idx.1 };

                if src.check_advance(&self) {
                    println!("({},{}) could advance.", idx.0, idx.1);
                    action_list.push(PlayerAction {
                        action: Action::Advance,
                        src: Location { m: idx.0, n: idx.1 },
                    });
                }

                if src.check_capture_left(&self) {
                    println!("({},{}) could capture left.", idx.0, idx.1);
                    action_list.push(PlayerAction {
                        action: Action::CaptureLeft,
                        src: Location { m: idx.0, n: idx.1 },
                    });
                }

                if src.check_capture_right(&self) {
                    println!("({},{}) could capture right.", idx.0, idx.1);
                    action_list.push(PlayerAction {
                        action: Action::CaptureRight,
                        src: Location { m: idx.0, n: idx.1 },
                    });
                }
            }
        }
        action_list
    }

    /// Return new `GameState` where piece at `src` has been advanced.
    ///
    /// Calculates destination relative to current player and calls `update(dst, src)`
    /// to create the new `GameState`.
    ///
    /// # Arguments
    ///
    /// `src` - the `Location` of the piece to be advanced
    pub fn advance(&self, src: Location) -> GameState {
        let new_m;
        match self.player {
            Player::Max => new_m = src.m + 1,
            Player::Min => new_m = src.m - 1,
        }

        let dst = Location { m: new_m, n: src.n };
        self.update(dst, src)
    }

    /// Return new `GameState` where piece at `src` has taken the opposing piece diagonally to the left.
    ///
    /// Calculates destination relative to current player and calls `update(dst, src)`
    /// to create the new `GameState`.
    ///
    /// # Arguments
    ///
    /// `src` - the `Location` of the piece initiating the capture
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

    /// Return new `GameState` where piece at `src` has taken the opposing piece diagonally to the right.
    ///
    /// Calculates destination relative to current player and calls `update(dst, src)`
    /// to create the new `GameState`.
    ///
    /// # Arguments
    ///
    /// `src` - the `Location` of the piece initiating the capture
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

    /// `GameState` is terminal if a board transversal has occured,
    /// or the current player is not able to move.
    pub fn is_terminal(&self) -> bool {
        for i in 0..3 {
            if self.board[(0, i)] == Player::Min.value()
                || self.board[(2, i)] == Player::Max.value()
            {
                return true;
            }
        }

        self.actions().is_empty()
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

    /// Return the `GameState` resulting from a given `PlayerAction`.
    ///
    /// # Arguments
    ///
    /// `a` - A `PlayerAction` to be applied to the current `GameState`
    pub fn result(&self, a: PlayerAction) -> GameState {
        match a.action {
            Action::Advance => self.advance(a.src),
            Action::CaptureLeft => self.capture_left(a.src),
            Action::CaptureRight => self.capture_right(a.src),
        }
    }

    /// Return `GameState` as a Vector.
    pub fn to_vector(&self) -> Vec<i8> {
        let b = self.board.clone();
        let mut vec_rep =  vec![self.player.value()];
        vec_rep.append(&mut b.into_raw_vec());
        println!("{:?}", vec_rep);
        vec_rep
    }

    /// Return a `GameState` with the piece originating in `src` moved to `dst`,
    /// and the empty `src` location filled with a zero.
    ///
    /// # Arguments
    ///
    /// `dst` - The new `Location`
    ///
    /// `src` - The original, current, `Location`
    fn update(&self, dst: Location, src: Location) -> GameState {
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
            "\nBoard:\n{:3}\n{:3}\n{:3}\nNext Player: {}\n",
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

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.m, self.n)
    }
}

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

pub struct PlayerAction {
    pub action: Action,
    pub src: Location,
}

impl fmt::Display for PlayerAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action: {} @ {}", self.action, self.src)
    }
}
