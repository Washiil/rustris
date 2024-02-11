use rand::prelude::*;
use std::{thread, time};

mod tetris;
mod tetris_piece;
use crate::{tetris::Tetris, tetris_piece::TetrisPieceType, tetris_piece::Tetromino};

fn main() {
    println!("Hello, Rustris!");
    let rustris = Tetris::new();
    rustris.print_board();

    let ten_millis = time::Duration::from_millis(50);
    // loop {
    //     rustris.print_board();
    //     thread::sleep(ten_millis);
    // }

    let mut tetromino = Tetromino::new(TetrisPieceType::L);

    println!("Original Matrix:");
    for row in tetromino.matrix().iter() {
        println!("{:?}", row);
    }
    println!("Rotated Matrix:");
    tetromino.rotate_clockwise();
    for row in tetromino.matrix().iter() {
        println!("{:?}", row);
    }
}
