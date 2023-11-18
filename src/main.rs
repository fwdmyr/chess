use std::fmt;
use std::ops;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum CatchAllError {
    InvalidPosition,
    BadParse,
}

impl fmt::Display for CatchAllError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CatchAllError::InvalidPosition => write!(f, "the position is invalid"),
            CatchAllError::BadParse => write!(f, "could not parse literal"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Color {
    White,
    Black,
    None,
}

#[derive(Debug, PartialEq)]
pub enum Piece {
    Pawn(Color, bool),
    Knight(Color),
    Bishop(Color),
    Rook(Color, bool),
    Queen(Color),
    King(Color, bool),
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        let (repr, color) = match self {
            Piece::Pawn(color, _) => ("p", color),
            Piece::Knight(color) => ("k", color),
            Piece::Bishop(color) => ("b", color),
            Piece::Rook(color, _) => ("r", color),
            Piece::Queen(color) => ("q", color),
            Piece::King(color, _) => ("k", color),
        };
        match color {
            Color::White => repr.to_uppercase(),
            Color::Black => repr.to_string(),
            Color::None => "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct File(usize);

#[derive(Debug, PartialEq)]
pub struct Rank(usize);

#[derive(Debug, PartialEq)]
pub enum Castle {
    Long,
    Short,
}

impl FromStr for Piece {
    type Err = CatchAllError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Piece::Knight(Color::None)),
            "B" => Ok(Piece::Bishop(Color::None)),
            "R" => Ok(Piece::Rook(Color::None, false)),
            "Q" => Ok(Piece::Queen(Color::None)),
            "K" => Ok(Piece::King(Color::None, false)),
            _ => Err(CatchAllError::BadParse),
        }
    }
}

impl FromStr for File {
    type Err = CatchAllError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(File(0)),
            "b" => Ok(File(1)),
            "c" => Ok(File(2)),
            "d" => Ok(File(3)),
            "e" => Ok(File(4)),
            "f" => Ok(File(5)),
            "g" => Ok(File(6)),
            "h" => Ok(File(7)),
            _ => Err(CatchAllError::BadParse),
        }
    }
}

impl FromStr for Rank {
    type Err = CatchAllError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Rank(0)),
            "2" => Ok(Rank(1)),
            "3" => Ok(Rank(2)),
            "4" => Ok(Rank(3)),
            "5" => Ok(Rank(4)),
            "6" => Ok(Rank(5)),
            "7" => Ok(Rank(6)),
            "8" => Ok(Rank(7)),
            _ => Err(CatchAllError::BadParse),
        }
    }
}

impl FromStr for Castle {
    type Err = CatchAllError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O-O" => Ok(Castle::Short),
            "O-O-O" => Ok(Castle::Long),
            _ => Err(CatchAllError::BadParse),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Piece(Piece),
    Capture,
    File(File),
    Rank(Rank),
    Check,
    Checkmate,
    Castle(Castle),
}

impl FromStr for Token {
    type Err = CatchAllError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Token::Capture),
            "+" => Ok(Token::Check),
            "#" => Ok(Token::Checkmate),
            _ => match Piece::from_str(&s) {
                Ok(piece) => Ok(Token::Piece(piece)),
                _ => match File::from_str(&s) {
                    Ok(file) => Ok(Token::File(file)),
                    _ => match Rank::from_str(&s) {
                        Ok(rank) => Ok(Token::Rank(rank)),
                        Err(_) => Err(CatchAllError::BadParse),
                    },
                },
            },
        }
    }
}

pub trait Tokenize {
    type TokenizeResult;
    fn tokenize(&self) -> Self::TokenizeResult;
}

impl Tokenize for &str {
    type TokenizeResult = Result<Vec<Token>, CatchAllError>;

    fn tokenize(&self) -> Self::TokenizeResult {
        // Castle is only token that is not a char.
        match Castle::from_str(&self) {
            Ok(castle) => Ok(vec![Token::Castle(castle)]),
            Err(_) => self
                .chars()
                .map(|ch| String::from(ch))
                .map(|s| Token::from_str(&s))
                .collect::<Self::TokenizeResult>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_capture() {
        let mut it = "dxe5".tokenize().unwrap().into_iter();
        assert_eq!(it.next(), Some(Token::File(File(3))));
        assert_eq!(it.next(), Some(Token::Capture));
        assert_eq!(it.next(), Some(Token::File(File(4))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn piece_capture() {
        let mut it = "Qxe5".tokenize().unwrap().into_iter();
        assert_eq!(it.next(), Some(Token::Piece(Piece::Queen(Color::None))));
        assert_eq!(it.next(), Some(Token::Capture));
        assert_eq!(it.next(), Some(Token::File(File(4))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn piece_move() {
        let mut it = "Re5".tokenize().unwrap().into_iter();
        assert_eq!(it.next(), Some(Token::Piece(Piece::Knight(Color::None))));
        assert_eq!(it.next(), Some(Token::File(File(4))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn file_qualified_move() {
        let mut it = "Rfe5".tokenize().unwrap().into_iter();
        assert_eq!(it.next(), Some(Token::Piece(Piece::Knight(Color::None))));
        assert_eq!(it.next(), Some(Token::File(File(5))));
        assert_eq!(it.next(), Some(Token::File(File(4))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn file_and_rank_qualified_move() {
        let mut it = "Rf5e5".tokenize().unwrap().into_iter();
        assert_eq!(it.next(), Some(Token::Piece(Piece::Knight(Color::None))));
        assert_eq!(it.next(), Some(Token::File(File(5))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), Some(Token::File(File(4))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn piece_move_with_check() {
        let mut it = "Re5+".tokenize().unwrap().into_iter();
        assert_eq!(it.next(), Some(Token::Piece(Piece::Knight(Color::None))));
        assert_eq!(it.next(), Some(Token::File(File(4))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), Some(Token::Check));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn piece_move_with_checkmate() {
        let mut it = "Re5#".tokenize().unwrap().into_iter();
        assert_eq!(it.next(), Some(Token::Piece(Piece::Knight(Color::None))));
        assert_eq!(it.next(), Some(Token::File(File(4))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), Some(Token::Checkmate));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn short_castle() {
        let mut it = "O-O".tokenize().unwrap().into_iter();
        assert_eq!(it.next(), Some(Token::Castle(Castle::Short)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn long_castle() {
        let mut it = "O-O-O".tokenize().unwrap().into_iter();
        assert_eq!(it.next(), Some(Token::Castle(Castle::Long)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn invalid_move_bad_parse_error() {
        let res = "Rzx9".tokenize();
        assert_eq!(res, Err(CatchAllError::BadParse));
    }
}

pub struct Position {
    row: u8,
    col: u8,
}

impl Position {
    fn new(row: u8, col: u8) -> Self {
        let position = Self { row, col };
        if !position.valid() {
            panic!("Invalid position ({}, {})", row, col);
        }
        position
    }

    fn valid(&self) -> bool {
        (self.row < 8) && (self.col < 8)
    }
}

impl Into<usize> for Position {
    fn into(self) -> usize {
        (8 * self.row + self.col) as usize
    }
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub enum Move {
    Regular(Direction, u8),
    Diagonal(Direction, Direction, u8),
    Jump(Direction, Direction),
}

pub struct Board {
    fields: [Option<Piece>; 64],
}

impl Board {
    fn new() -> Self {
        const INIT: Option<Piece> = None;
        let mut board = Self { fields: [INIT; 64] };
        // First rank.
        board[Position::new(0, 0)] = Some(Piece::Rook(Color::White, false));
        board[Position::new(0, 1)] = Some(Piece::Knight(Color::White));
        board[Position::new(0, 2)] = Some(Piece::Bishop(Color::White));
        board[Position::new(0, 3)] = Some(Piece::Queen(Color::White));
        board[Position::new(0, 4)] = Some(Piece::King(Color::White, false));
        board[Position::new(0, 5)] = Some(Piece::Bishop(Color::White));
        board[Position::new(0, 6)] = Some(Piece::Knight(Color::White));
        board[Position::new(0, 7)] = Some(Piece::Rook(Color::White, false));
        // Second rank.
        board[Position::new(1, 0)] = Some(Piece::Pawn(Color::White, false));
        board[Position::new(1, 1)] = Some(Piece::Pawn(Color::White, false));
        board[Position::new(1, 2)] = Some(Piece::Pawn(Color::White, false));
        board[Position::new(1, 3)] = Some(Piece::Pawn(Color::White, false));
        board[Position::new(1, 4)] = Some(Piece::Pawn(Color::White, false));
        board[Position::new(1, 5)] = Some(Piece::Pawn(Color::White, false));
        board[Position::new(1, 6)] = Some(Piece::Pawn(Color::White, false));
        board[Position::new(1, 7)] = Some(Piece::Pawn(Color::White, false));
        // Seventh rank.
        board[Position::new(6, 0)] = Some(Piece::Pawn(Color::Black, false));
        board[Position::new(6, 1)] = Some(Piece::Pawn(Color::Black, false));
        board[Position::new(6, 2)] = Some(Piece::Pawn(Color::Black, false));
        board[Position::new(6, 3)] = Some(Piece::Pawn(Color::Black, false));
        board[Position::new(6, 4)] = Some(Piece::Pawn(Color::Black, false));
        board[Position::new(6, 5)] = Some(Piece::Pawn(Color::Black, false));
        board[Position::new(6, 6)] = Some(Piece::Pawn(Color::Black, false));
        board[Position::new(6, 7)] = Some(Piece::Pawn(Color::Black, false));
        // Eight rank.
        board[Position::new(7, 0)] = Some(Piece::Rook(Color::Black, false));
        board[Position::new(7, 1)] = Some(Piece::Knight(Color::Black));
        board[Position::new(7, 2)] = Some(Piece::Bishop(Color::Black));
        board[Position::new(7, 3)] = Some(Piece::Queen(Color::Black));
        board[Position::new(7, 4)] = Some(Piece::King(Color::Black, false));
        board[Position::new(7, 5)] = Some(Piece::Bishop(Color::Black));
        board[Position::new(7, 6)] = Some(Piece::Knight(Color::Black));
        board[Position::new(7, 7)] = Some(Piece::Rook(Color::Black, false));
        board
    }
}

impl ops::IndexMut<Position> for Board {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        let i: usize = position.into();
        &mut self.fields[i]
    }
}

impl ops::Index<Position> for Board {
    type Output = Option<Piece>;

    fn index(&self, position: Position) -> &Self::Output {
        let i: usize = position.into();
        &self.fields[i]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "---------------------------------\n").unwrap();
        (0..8).into_iter().rev().for_each(|i| {
            let row = (0..8).into_iter().fold(String::from("| "), |mut acc, j| {
                acc.push_str(&format!(
                    "{} | ",
                    self[Position::new(i, j)]
                        .as_ref()
                        .map_or(" ".to_string(), |piece| piece.to_string())
                ));
                acc
            });
            write!(f, "{}\n", row).unwrap();
            write!(f, "---------------------------------\n").unwrap();
        });
        Ok(())
    }
}

fn main() {
    let board = Board::new();

    println!("{}", board);
}
