use crate::error::CatchAllError;
use crate::piece::{Color, Piece, State};
use crate::position::Position;
use crate::r#move::{Action, Move};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MoveCache {
    pub from: Position,
    pub to: Position,
    pub captured: Option<Piece>,
}

impl MoveCache {
    pub fn new(from: Position, to: Position, captured: Option<Piece>) -> Self {
        Self {
            from,
            to,
            captured: captured,
        }
    }
}

pub struct Board {
    pieces: HashMap<Position, Piece>,
    cache: Option<MoveCache>,
}

impl Board {
    #[rustfmt::skip]
    pub fn new() -> Self {
        let mut board = Self {
            pieces: HashMap::new(),
            cache: None,
        };

        board.pieces.insert(Position::new(0, 0), Piece::Rook( Color::White, State::Initial));
        board.pieces.insert(Position::new(1, 0), Piece::Knight( Color::White));
        board.pieces.insert(Position::new(2, 0), Piece::Bishop( Color::White));
        board.pieces.insert(Position::new(3, 0), Piece::Queen( Color::White));
        board.pieces.insert(Position::new(4, 0), Piece::King( Color::White, State::Initial));
        board.pieces.insert(Position::new(5, 0), Piece::Bishop( Color::White));
        board.pieces.insert(Position::new(6, 0), Piece::Knight( Color::White));
        board.pieces.insert(Position::new(7, 0), Piece::Rook( Color::White, State::Initial));
        board.pieces.insert(Position::new(0, 1), Piece::Pawn( Color::White, State::Initial));
        board.pieces.insert(Position::new(1, 1), Piece::Pawn( Color::White, State::Initial));
        board.pieces.insert(Position::new(2, 1), Piece::Pawn( Color::White, State::Initial));
        board.pieces.insert(Position::new(3, 1), Piece::Pawn( Color::White, State::Initial));
        board.pieces.insert(Position::new(4, 1), Piece::Pawn( Color::White, State::Initial));
        board.pieces.insert(Position::new(5, 1), Piece::Pawn( Color::White, State::Initial));
        board.pieces.insert(Position::new(6, 1), Piece::Pawn( Color::White, State::Initial));
        board.pieces.insert(Position::new(7, 1), Piece::Pawn( Color::White, State::Initial));
        board.pieces.insert(Position::new(0, 6), Piece::Pawn( Color::Black, State::Initial));
        board.pieces.insert(Position::new(1, 6), Piece::Pawn( Color::Black, State::Initial));
        board.pieces.insert(Position::new(2, 6), Piece::Pawn( Color::Black, State::Initial));
        board.pieces.insert(Position::new(3, 6), Piece::Pawn( Color::Black, State::Initial));
        board.pieces.insert(Position::new(4, 6), Piece::Pawn( Color::Black, State::Initial));
        board.pieces.insert(Position::new(5, 6), Piece::Pawn( Color::Black, State::Initial));
        board.pieces.insert(Position::new(6, 6), Piece::Pawn( Color::Black, State::Initial));
        board.pieces.insert(Position::new(7, 6), Piece::Pawn( Color::Black, State::Initial));
        board.pieces.insert(Position::new(0, 7), Piece::Rook( Color::Black, State::Initial));
        board.pieces.insert(Position::new(1, 7), Piece::Knight( Color::Black));
        board.pieces.insert(Position::new(2, 7), Piece::Bishop( Color::Black));
        board.pieces.insert(Position::new(3, 7), Piece::Queen( Color::Black));
        board.pieces.insert(Position::new(4, 7), Piece::King( Color::Black, State::Initial));
        board.pieces.insert(Position::new(5, 7), Piece::Bishop( Color::Black));
        board.pieces.insert(Position::new(6, 7), Piece::Knight( Color::Black));
        board.pieces.insert(Position::new(7, 7), Piece::Rook( Color::Black, State::Initial));

        board
    }

    #[rustfmt::skip]
    pub fn advance(&mut self, color: &Color, from: &Position, to: &Position) -> Result<(), CatchAllError> {
        self.assess_turn(color, from, to)?;
        self.update(from, to)?;

        Ok(())
    }

    pub fn at(&self, pos: &Position) -> Result<&Piece, CatchAllError> {
        self.pieces
            .get(pos)
            .map_or(Err(CatchAllError::EmptyField), |p| Ok(p))
    }

    pub fn king(&self, color: &Color) -> Result<(&Position, &Piece), CatchAllError> {
        self.pieces
            .iter()
            .find(|(_, v)| match v {
                Piece::King(c, _) if c == color => true,
                _ => false,
            })
            .ok_or(CatchAllError::NoKing)
    }

    pub fn in_check(&self, color: &Color) -> Result<bool, CatchAllError> {
        let (pos, _) = self.king(color)?;
        Ok(self.pieces.iter().any(|(k, v)| {
            &v.color() != color
                && v.can_reach(&Move::new(k, pos, Action::Regular)).is_ok()
                && self
                    .assess_move(k, &Move::new(k, pos, Action::Regular))
                    .is_ok()
        }))
    }

    fn piece_at(&self, pos: &Position, color: &Color) -> Result<&Piece, CatchAllError> {
        self.pieces
            .get(pos)
            .map_or(Err(CatchAllError::EmptyField), |p| {
                (&p.color() == color)
                    .then(|| p)
                    .ok_or(CatchAllError::EmptyField)
            })
    }

    fn action(&self, pos: &Position, color: &Color) -> Result<Action, CatchAllError> {
        self.pieces.get(pos).map_or(Ok(Action::Regular), |p| {
            (&p.color() != color)
                .then(|| Action::Capture)
                .ok_or(CatchAllError::EmptyField)
        })
    }

    fn has_piece(&self, pos: &Position) -> Result<(), CatchAllError> {
        self.pieces
            .contains_key(&pos)
            .eq(&false)
            .then(|| ())
            .ok_or(CatchAllError::BlockedPath)
    }

    fn update(&mut self, from: &Position, to: &Position) -> Result<(), CatchAllError> {
        let captured = self.pieces.get(to).map(|p| p.clone());
        let mut piece = self.pieces.remove(from).ok_or(CatchAllError::EmptyField)?;
        piece.update();
        self.pieces.insert(to.clone(), piece);

        self.cache = Some(MoveCache::new(from.clone(), to.clone(), captured));

        Ok(())
    }

    fn revert(&mut self) -> Result<(), CatchAllError> {
        let cache = self.cache.clone().ok_or(CatchAllError::EmptyMoveCache)?;
        let piece = self
            .pieces
            .remove(&cache.to)
            .ok_or(CatchAllError::EmptyField)?;
        self.pieces.insert(cache.from, piece);

        if let Some(captured) = cache.captured {
            self.pieces.insert(cache.to, captured);
        }

        self.cache = None;

        Ok(())
    }

    fn assess_move(&self, pos: &Position, mv: &Move) -> Result<(), CatchAllError> {
        pos.path(mv)?
            .iter()
            .try_fold((), |_, position| self.has_piece(position))
    }

    #[rustfmt::skip]
    fn resolve_check(&mut self, from: &Position, to: &Position, color: &Color) -> Result<(), CatchAllError> {
        self.update(from, to)?;

        let res = self.in_check(color);

        self.revert()?;

        res?.then(|| ()).map_or(Ok(()), |_| Err(CatchAllError::InCheck))?;

        Ok(())
    }

    #[rustfmt::skip]
    fn assess_turn(&mut self, color: &Color, from: &Position, to: &Position) -> Result<(), CatchAllError> {
        // Check if piece of correct color is at from position.
        let piece = self.piece_at(from, color)?;

        // Check if piece is at to.
        // If piece of opposite color, the action will be capture.
        // If piece of same color, the path is blocked.
        let action = self.action(to, color)?;
        let mv = Move::new(from, to, action);

        // Check if piece can reach the to position from the from position.
        piece.can_reach(&mv)?;

        // Check if the path taken by move from to is unobstructed.
        self.assess_move(from, &mv)?;

        // Check if the king would be in check after the move.
        self.resolve_check(from ,to, color)?;

        Ok(())
    }
}
