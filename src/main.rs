use rand::prelude::*;

#[derive(Clone, Copy)]
enum TetrisPiece {
    O,
    I,
    S,
    Z,
    L,
    J,
    T
}

impl TetrisPiece {
    pub fn random_piece() -> TetrisPiece {
        let pieces = [
            TetrisPiece::O,
            TetrisPiece::I,
            TetrisPiece::S,
            TetrisPiece::Z,
            TetrisPiece::L,
            TetrisPiece::J,
            TetrisPiece::T,
        ];
    
        let mut rng = rand::thread_rng();
        return *pieces.choose(&mut rng).unwrap();
    }
}

struct Tetris {
    board: [[u32; 10]; 20],
    upcoming: [TetrisPiece; 5]
}

impl Tetris {
    pub fn new() -> Self {
        return Tetris {
            board: [[0; 10]; 20],
            upcoming: [
                TetrisPiece::random_piece(),
                TetrisPiece::random_piece(),
                TetrisPiece::random_piece(),
                TetrisPiece::random_piece(),
                TetrisPiece::random_piece()
            ],
        }
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

fn main() {
    println!("Hello, Rustris!");
    let rustris = Tetris::new();
    rustris.print_board();
}
