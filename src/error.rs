use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CatchAllError {
    EmptyMoveCache,
    NoKing,
    InCheck,
    InvalidPath,
    BlockedPath,
    EmptyField,
    UnreachableField,
    InvalidTurn,
}

impl fmt::Display for CatchAllError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CatchAllError::EmptyMoveCache => write!(f, "the move cache is empty"),
            CatchAllError::NoKing => write!(f, "the king does not exist"),
            CatchAllError::InCheck => write!(f, "the king is in check"),
            CatchAllError::InvalidPath => write!(f, "the path is invalid"),
            CatchAllError::BlockedPath => write!(f, "the path is blocked"),
            CatchAllError::EmptyField => write!(f, "the field is empty"),
            CatchAllError::UnreachableField => write!(f, "the field is unreachable"),
            CatchAllError::InvalidTurn => write!(f, "the turn is invalid"),
        }
    }
}
