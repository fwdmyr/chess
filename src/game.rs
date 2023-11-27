use crate::board::Board;
use crate::error::CatchAllError;
use crate::piece::Color;
use crate::piece::Piece;
use crate::position::Position;

#[derive(Debug, Clone, Copy)]
pub enum Turn {
    New(Color),
    Select(Color, Position),
}

pub struct Game {
    board: Board,
    turn: Turn,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Turn::New(Color::White),
        }
    }

    pub fn reset_turn(&mut self) {
        self.turn = match self.turn {
            Turn::New(color) => Turn::New(color),
            Turn::Select(color, _) => Turn::New(color),
        }
    }

    pub fn advance(&mut self, pos: &Position) -> Result<(), CatchAllError> {
        self.turn = match self.turn {
            Turn::New(_) => self.select(pos)?,
            Turn::Select(_, _) => self.play(pos)?,
        };

        Ok(())
    }

    pub fn at(&self, pos: &Position) -> Result<&Piece, CatchAllError> {
        self.board.at(pos)
    }

    pub fn turn(&self) -> Turn {
        self.turn
    }

    fn select(&self, pos: &Position) -> Result<Turn, CatchAllError> {
        match self.turn {
            Turn::New(color) => self
                .board
                .at(&pos)?
                .color()
                .eq(&color)
                .then(|| ())
                .map_or(Err(CatchAllError::InvalidTurn), |_| {
                    Ok(Turn::Select(color, pos.clone()))
                }),
            _ => Err(CatchAllError::InvalidTurn),
        }
    }

    fn play(&mut self, pos: &Position) -> Result<Turn, CatchAllError> {
        match self.turn {
            Turn::Select(color, from) => self.board.advance(&color, &from, pos).map_or(
                Err(CatchAllError::InvalidTurn),
                |_| match color {
                    Color::White => Ok(Turn::New(Color::Black)),
                    Color::Black => Ok(Turn::New(Color::White)),
                },
            ),
            _ => Err(CatchAllError::InvalidTurn),
        }
    }
}
