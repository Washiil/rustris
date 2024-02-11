use rand::prelude::*;
use std::{thread, time};

use crate::tetris_piece::Tetromino;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

pub struct Tetris {
    board: [[u32; BOARD_WIDTH]; BOARD_HEIGHT],
    // Top left of any given piece
    piece_x: u32,
    piece_y: u32, 
    upcoming: [Tetromino; 28],
}

impl Tetris {
    pub fn new() -> Self {
        let upcoming = Tetromino::generate_tetris_set();

        return Tetris {
            board: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
            piece_x: 3,
            piece_y: 0,
            upcoming,
        };
    }

    fn drop_piece(&mut self) {
        if self.piece_y > 0 {
            self.piece_y -= 1;
        }
    }

    // Should be run every time the length of the current upcoming is 0
    pub fn refill_upcoming(&mut self) {
        self.upcoming = Tetromino::generate_tetris_set();
    }

    pub fn print_board(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }
    }
}
