/// Contains functions defining the game and valid moves.

use peroxide::prelude::*;

/// m by n index values within the game matrix
#[derive(Debug)]
struct Location{
    m: u8,
    n: u8,
}

pub fn new_game() {
    let board = matrix(c!(1,1,1,0,0,0,-1,-1,-1), 3, 3, Row);
    println!("{}", board);
}


