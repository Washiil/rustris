use rand::prelude::*;

use crate::tetris::Tetris;

#[derive(Debug, Clone)]
pub enum TetrisPieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrisPieceType {
    pub fn to_matrix(self) -> Vec<Vec<u8>> {
        let matrix = match self {
            TetrisPieceType::I => vec![
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            TetrisPieceType::O => vec![vec![2, 2], vec![2, 2]],
            TetrisPieceType::T => vec![vec![0, 3, 0], vec![3, 3, 3], vec![0, 0, 0]],
            TetrisPieceType::S => vec![vec![0, 4, 4], vec![4, 4, 0], vec![0, 0, 0]],
            TetrisPieceType::Z => vec![vec![5, 5, 0], vec![0, 5, 5], vec![0, 0, 0]],
            TetrisPieceType::J => vec![vec![0, 6, 0], vec![0, 6, 0], vec![6, 6, 0]],
            TetrisPieceType::L => vec![vec![0, 7, 0], vec![0, 7, 0], vec![0, 7, 7]],
        };
        return matrix;
    }
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    matrix: Vec<Vec<u8>>,
}

impl Tetromino {
    pub fn new(block_type: TetrisPieceType) -> Tetromino {
        let matrix = block_type.to_matrix();
        return Tetromino { matrix };
    }

    pub fn matrix(&self) -> &Vec<Vec<u8>> {
        return &self.matrix;
    }

    pub fn rotate_clockwise(&mut self) {
        let mut new_matrix = vec![vec![0; self.matrix.len()]; self.matrix[0].len()];
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[0].len() {
                new_matrix[j][self.matrix.len() - 1 - i] = self.matrix[i][j];
            }
        }
        self.matrix = new_matrix;
    }

    // This algoithm is not perfect, but it's good enough for now
    pub fn generate_tetris_set() -> [Tetromino; 28] {
        let mut pieces = [
            Tetromino::new(TetrisPieceType::I),
            Tetromino::new(TetrisPieceType::I),
            Tetromino::new(TetrisPieceType::I),
            Tetromino::new(TetrisPieceType::I),
            Tetromino::new(TetrisPieceType::O),
            Tetromino::new(TetrisPieceType::O),
            Tetromino::new(TetrisPieceType::O),
            Tetromino::new(TetrisPieceType::O),
            Tetromino::new(TetrisPieceType::S),
            Tetromino::new(TetrisPieceType::S),
            Tetromino::new(TetrisPieceType::S),
            Tetromino::new(TetrisPieceType::S),
            Tetromino::new(TetrisPieceType::Z),
            Tetromino::new(TetrisPieceType::Z),
            Tetromino::new(TetrisPieceType::Z),
            Tetromino::new(TetrisPieceType::Z),
            Tetromino::new(TetrisPieceType::L),
            Tetromino::new(TetrisPieceType::L),
            Tetromino::new(TetrisPieceType::L),
            Tetromino::new(TetrisPieceType::L),
            Tetromino::new(TetrisPieceType::J),
            Tetromino::new(TetrisPieceType::J),
            Tetromino::new(TetrisPieceType::J),
            Tetromino::new(TetrisPieceType::J),
            Tetromino::new(TetrisPieceType::T),
            Tetromino::new(TetrisPieceType::T),
            Tetromino::new(TetrisPieceType::T),
            Tetromino::new(TetrisPieceType::T),
        ];

        pieces.shuffle(&mut rand::thread_rng());
        return pieces;
    }
}
