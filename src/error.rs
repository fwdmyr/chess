use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CatchAllError {
    InvalidPath,
    BlockedPath,
    EmptyField,
    UnreachableField,
    InvalidTurn,
}

impl fmt::Display for CatchAllError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CatchAllError::InvalidPath => write!(f, "the path is invalid"),
            CatchAllError::BlockedPath => write!(f, "the path is blocked"),
            CatchAllError::EmptyField => write!(f, "the field is empty"),
            CatchAllError::UnreachableField => write!(f, "the field is unreachable"),
            CatchAllError::InvalidTurn => write!(f, "the turn is invalid"),
        }
    }
}
