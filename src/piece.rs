use crate::error::CatchAllError;
use crate::position::Position;
use crate::r#move::{Action, Direction, Move};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl From<Position> for Color {
    fn from(position: Position) -> Self {
        if (position.file() + position.rank()) % 2 == 0 {
            Color::Black
        } else {
            Color::White
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveCounter(pub u32);

impl MoveCounter {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn decrement(&mut self) {
        self.0 -= 1;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Piece {
    Pawn(Color, MoveCounter),
    Knight(Color),
    Bishop(Color),
    Rook(Color, MoveCounter),
    Queen(Color),
    King(Color, MoveCounter),
}

impl Piece {
    pub fn update(&mut self) {
        match self {
            Piece::Pawn(_, ref mut counter)
            | Piece::Rook(_, ref mut counter)
            | Piece::King(_, ref mut counter) => {
                counter.increment();
            }
            _ => (),
        }
    }

    pub fn revert(&mut self) {
        match self {
            Piece::Pawn(_, ref mut counter)
            | Piece::Rook(_, ref mut counter)
            | Piece::King(_, ref mut counter) => {
                counter.decrement();
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

    pub fn all_moves(&self, from: &Position) -> Vec<Position> {
        (0..8)
            .zip(0..8)
            .filter_map(|(i, j)| {
                let to = Position::new(i, j);
                match self {
                    Piece::Pawn(_, _) => {
                        let regular_mv = Move::new(from, &to, Action::Regular);
                        let capture_mv = Move::new(from, &to, Action::Capture);

                        self.can_reach(&regular_mv)
                            .map_or(None, |_| Some(to))
                            .and_then(|_| self.can_reach(&capture_mv).map_or(None, |_| Some(to)))
                    }
                    _ => {
                        let mv = Move::new(from, &to, Action::Regular);
                        self.can_reach(&mv).map_or(None, |_| Some(to))
                    }
                }
            })
            .collect()
    }

    pub fn can_reach(&self, mv: &Move) -> Result<(), CatchAllError> {
        match self {
            Piece::Pawn(color, counter) => Piece::can_reach_pawn(mv, color, counter),
            Piece::Knight(_) => Piece::can_reach_knight(mv),
            Piece::Bishop(_) => Piece::can_reach_bishop(mv),
            Piece::Rook(_, _) => Piece::can_reach_rook(mv),
            Piece::Queen(_) => Piece::can_reach_queen(mv),
            Piece::King(_, state) => Piece::can_reach_king(mv, state),
        }
    }

    #[rustfmt::skip]
    fn can_reach_pawn(mv: &Move, color: &Color, counter: &MoveCounter) -> Result<(), CatchAllError> {
        match (mv, color, counter) {
            (Move::Straight(Direction::Up, 2, Action::Regular), Color::White, MoveCounter(0)) => Ok(()),
            (Move::Straight(Direction::Up, 1, Action::Regular), Color::White, _) => Ok(()),
            (Move::Diagonal(Direction::Up, Direction::Left | Direction::Right, 1, Action::Capture), Color::White, _) => Ok(()),
            (Move::Straight(Direction::Down, 2, Action::Regular), Color::Black, MoveCounter(0)) => Ok(()),
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
    fn can_reach_king(mv: &Move, counter: &MoveCounter) -> Result<(), CatchAllError> {
        match (mv, counter) {
            (Move::Straight(_, 1, _), _) => Ok(()),
            (Move::Diagonal(_, _, 1, _), _) => Ok(()),
            (Move::Straight(Direction::Left, 2, Action::Regular), MoveCounter(0)) => Ok(()),
            (Move::Straight(Direction::Right, 2, Action::Regular), MoveCounter(0)) => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
        }
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self {
            // Piece::Pawn(_, _) => "♟".to_string(),
            // Piece::Knight(_) => "♞".to_string(),
            // Piece::Bishop(_) => "♝".to_string(),
            // Piece::Rook(_, _) => "♜".to_string(),
            // Piece::Queen(_) => "♛".to_string(),
            // Piece::King(_, _) => "♚".to_string(),
            Piece::Pawn(_, _) => "P".to_string(),
            Piece::Knight(_) => "N".to_string(),
            Piece::Bishop(_) => "B".to_string(),
            Piece::Rook(_, _) => "R".to_string(),
            Piece::Queen(_) => "Q".to_string(),
            Piece::King(_, _) => "K".to_string(),
        }
    }
}
