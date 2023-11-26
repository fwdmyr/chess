use std::io;
use std::str::FromStr;

use crate::board::Board;
use crate::error::CatchAllError;
use crate::piece::Color;
use crate::position::Position;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), CatchAllError> {
        println!("The game has started.");

        let mut color = Color::White;

        loop {
            match color {
                Color::White => println!("White to play."),
                Color::Black => println!("Black to play."),
            }

            let mut turn = String::new();

            println!("Enter turn.");

            io::stdin()
                .read_line(&mut turn)
                .ok()
                .expect("Failed to read line.");

            let from = Position::from_str(&turn[0..2]).unwrap();
            let to = Position::from_str(&turn[2..4]).unwrap();

            let res = self.board.advance(&color, &from, &to);

            match res {
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
                Ok(_) => (),
            }

            color = match color {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
        }
    }
}
