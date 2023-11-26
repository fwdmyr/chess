use crate::error::CatchAllError;
use crate::r#move::{Action, Direction, Move};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Initial,
    Moved,
}

#[derive(Debug, PartialEq)]
pub enum Piece {
    Pawn(Color, State),
    Knight(Color),
    Bishop(Color),
    Rook(Color, State),
    Queen(Color),
    King(Color, State),
}

impl Piece {
    pub fn update(&mut self) {
        match self {
            Piece::Pawn(_, ref mut state)
            | Piece::Rook(_, ref mut state)
            | Piece::King(_, ref mut state) => {
                *state = State::Moved;
            }
            _ => (),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Piece::Pawn(color, _) => color.clone(),
            Piece::Knight(color) => color.clone(),
            Piece::Bishop(color) => color.clone(),
            Piece::Rook(color, _) => color.clone(),
            Piece::Queen(color) => color.clone(),
            Piece::King(color, _) => color.clone(),
        }
    }

    pub fn can_reach(&self, mv: &Move) -> Result<(), CatchAllError> {
        match self {
            Piece::Pawn(color, state) => Piece::can_reach_pawn(mv, color, state),
            Piece::Knight(_) => Piece::can_reach_knight(mv),
            Piece::Bishop(_) => Piece::can_reach_bishop(mv),
            Piece::Rook(_, _) => Piece::can_reach_rook(mv),
            Piece::Queen(_) => Piece::can_reach_queen(mv),
            Piece::King(_, state) => Piece::can_reach_king(mv, state),
        }
    }

    #[rustfmt::skip]
    fn can_reach_pawn(mv: &Move, color: &Color, state: &State) -> Result<(), CatchAllError> {
        println!("{:?}", mv);
        match (mv, color, state) {
            (Move::Straight(Direction::Up, 2, Action::Regular), Color::White, State::Initial) => Ok(()),
            (Move::Straight(Direction::Up, 1, Action::Regular), Color::White, _) => Ok(()),
            (Move::Diagonal(Direction::Up, Direction::Left | Direction::Right, 1, Action::Capture), Color::White, _) => Ok(()),
            (Move::Straight(Direction::Down, 2, Action::Regular), Color::Black, State::Initial) => Ok(()),
            (Move::Straight(Direction::Down, 1, Action::Regular), Color::Black, _) => Ok(()),
            (Move::Diagonal(Direction::Down, Direction::Left | Direction::Right, 1, Action::Capture), Color::Black, _) => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
        }
    }

    #[rustfmt::skip]
    fn can_reach_knight(mv: &Move) -> Result<(), CatchAllError> {
        match mv {
            Move::Jump(_) => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
        }
    }

    #[rustfmt::skip]
    fn can_reach_bishop(mv: &Move) -> Result<(), CatchAllError> {
        match mv {
            Move::Diagonal(_, _, _, _) => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
        }
    }

    #[rustfmt::skip]
    fn can_reach_rook(mv: &Move) -> Result<(), CatchAllError> {
        match mv {
            Move::Straight(_, _, _) => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
        }
    }

    #[rustfmt::skip]
    fn can_reach_queen(mv: &Move) -> Result<(), CatchAllError> {
        match mv {
            Move::Straight(_, _, _) => Ok(()),
            Move::Diagonal(_, _, _, _) => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
        }
    }

    #[rustfmt::skip]
    fn can_reach_king(mv: &Move, state: &State) -> Result<(), CatchAllError> {
        match (mv, state) {
            (Move::Straight(_, 1, _), _) => Ok(()),
            (Move::Diagonal(_, _, 1, _), _) => Ok(()),
            (Move::Straight(Direction::Left, 3, Action::Regular), State::Initial) => Ok(()),
            (Move::Straight(Direction::Right, 2, Action::Regular), State::Initial) => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
        }
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self {
            Piece::Pawn(_, _) => "♟".to_string(),
            Piece::Knight(_) => "♞".to_string(),
            Piece::Bishop(_) => "♝".to_string(),
            Piece::Rook(_, _) => "♜".to_string(),
            Piece::Queen(_) => "♛".to_string(),
            Piece::King(_, _) => "♚".to_string(),
        }
    }
}
