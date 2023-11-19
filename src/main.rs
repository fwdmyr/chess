use std::fmt;
use std::io;
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
}

#[derive(Debug, PartialEq)]
pub struct PieceData {
    position: Position,
    color: Color,
}

impl PieceData {
    fn new(position: Position, color: Color) -> Self {
        Self { position, color }
    }
}

impl Default for PieceData {
    fn default() -> Self {
        Self {
            position: Position::new(0, 0),
            color: Color::White,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Piece {
    Pawn(PieceData, bool),
    Knight(PieceData),
    Bishop(PieceData),
    Rook(PieceData, bool),
    Queen(PieceData),
    King(PieceData, bool),
}

impl Piece {
    pub fn position(&self) -> &Position {
        match self {
            Piece::Pawn(data, _) => &data.position,
            Piece::Knight(data) => &data.position,
            Piece::Bishop(data) => &data.position,
            Piece::Rook(data, _) => &data.position,
            Piece::Queen(data) => &data.position,
            Piece::King(data, _) => &data.position,
        }
    }

    pub fn color(&self) -> &Color {
        match self {
            Piece::Pawn(data, _) => &data.color,
            Piece::Knight(data) => &data.color,
            Piece::Bishop(data) => &data.color,
            Piece::Rook(data, _) => &data.color,
            Piece::Queen(data) => &data.color,
            Piece::King(data, _) => &data.color,
        }
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self {
            Piece::Pawn(data, _) => match data.color {
                Color::White => "♟".to_string(),
                Color::Black => "♙".to_string(),
            },
            Piece::Knight(data) => match data.color {
                Color::White => "♞".to_string(),
                Color::Black => "♘".to_string(),
            },
            Piece::Bishop(data) => match data.color {
                Color::White => "♝".to_string(),
                Color::Black => "♗".to_string(),
            },
            Piece::Rook(data, _) => match data.color {
                Color::White => "♜".to_string(),
                Color::Black => "♖".to_string(),
            },
            Piece::Queen(data) => match data.color {
                Color::White => "♛".to_string(),
                Color::Black => "♕".to_string(),
            },
            Piece::King(data, _) => match data.color {
                Color::White => "♚".to_string(),
                Color::Black => "♔".to_string(),
            },
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
            "N" => Ok(Piece::Knight(PieceData::default())),
            "B" => Ok(Piece::Bishop(PieceData::default())),
            "R" => Ok(Piece::Rook(PieceData::default(), false)),
            "Q" => Ok(Piece::Queen(PieceData::default())),
            "K" => Ok(Piece::King(PieceData::default(), false)),
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

impl Tokenize for String {
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

fn validate(tokens: &mut Vec<Token>) {
    match tokens.pop() {
        Some(Token::Piece(piece)) => println!("Promotion to {}", piece.to_string()),
        Some(Token::Rank(Rank(rank))) => match tokens.pop() {
            Some(Token::File(File(file))) => {
                println!("Destination {:?}", Position::new(rank, file))
            }
            _ => panic!("Bad"),
        },
        _ => panic!("Bad"),
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
        assert_eq!(
            it.next(),
            Some(Token::Piece(Piece::Queen(PieceData::default())))
        );
        assert_eq!(it.next(), Some(Token::Capture));
        assert_eq!(it.next(), Some(Token::File(File(4))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn piece_move() {
        let mut it = "Re5".tokenize().unwrap().into_iter();
        assert_eq!(
            it.next(),
            Some(Token::Piece(Piece::Knight(PieceData::default())))
        );
        assert_eq!(it.next(), Some(Token::File(File(4))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn file_qualified_move() {
        let mut it = "Rfe5".tokenize().unwrap().into_iter();
        assert_eq!(
            it.next(),
            Some(Token::Piece(Piece::Knight(PieceData::default())))
        );
        assert_eq!(it.next(), Some(Token::File(File(5))));
        assert_eq!(it.next(), Some(Token::File(File(4))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn file_and_rank_qualified_move() {
        let mut it = "Rf5e5".tokenize().unwrap().into_iter();
        assert_eq!(
            it.next(),
            Some(Token::Piece(Piece::Knight(PieceData::default())))
        );
        assert_eq!(it.next(), Some(Token::File(File(5))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), Some(Token::File(File(4))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn piece_move_with_check() {
        let mut it = "Re5+".tokenize().unwrap().into_iter();
        assert_eq!(
            it.next(),
            Some(Token::Piece(Piece::Knight(PieceData::default())))
        );
        assert_eq!(it.next(), Some(Token::File(File(4))));
        assert_eq!(it.next(), Some(Token::Rank(Rank(4))));
        assert_eq!(it.next(), Some(Token::Check));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn piece_move_with_checkmate() {
        let mut it = "Re5#".tokenize().unwrap().into_iter();
        assert_eq!(
            it.next(),
            Some(Token::Piece(Piece::Knight(PieceData::default())))
        );
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

#[derive(Debug, PartialEq)]
pub struct Position {
    file: usize,
    rank: usize,
}

impl Position {
    fn new(file: usize, rank: usize) -> Self {
        let position = Self { file, rank };
        assert!(position.valid(), "Invalid position ({}, {})", file, rank);
        position
    }

    fn valid(&self) -> bool {
        (self.file < 8) && (self.rank < 8)
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
    white_pieces: Vec<Piece>,
    black_pieces: Vec<Piece>,
}

impl Board {
    #[rustfmt::skip]
    fn new() -> Self {
        let mut board = Self {
            white_pieces: Vec::new(),
            black_pieces: Vec::new(),
        };

        board.white_pieces.push(Piece::Rook(PieceData::new(Position::new(0, 0), Color::White), false));
        board.white_pieces.push(Piece::Knight(PieceData::new(Position::new(1, 0), Color::White)));
        board.white_pieces.push(Piece::Bishop(PieceData::new(Position::new(2, 0), Color::White)));
        board.white_pieces.push(Piece::Queen(PieceData::new(Position::new(3, 0), Color::White)));
        board.white_pieces.push(Piece::King(PieceData::new(Position::new(4, 0), Color::White), false));
        board.white_pieces.push(Piece::Bishop(PieceData::new(Position::new(5, 0), Color::White)));
        board.white_pieces.push(Piece::Knight(PieceData::new(Position::new(6, 0), Color::White)));
        board.white_pieces.push(Piece::Rook(PieceData::new(Position::new(7, 0), Color::White), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(0, 1), Color::White), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(1, 1), Color::White), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(2, 1), Color::White), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(3, 1), Color::White), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(4, 1), Color::White), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(5, 1), Color::White), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(6, 1), Color::White), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(7, 1), Color::White), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(0, 6), Color::Black), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(1, 6), Color::Black), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(2, 6), Color::Black), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(3, 6), Color::Black), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(4, 6), Color::Black), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(5, 6), Color::Black), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(6, 6), Color::Black), false));
        board.white_pieces.push(Piece::Pawn(PieceData::new(Position::new(7, 6), Color::Black), false));
        board.white_pieces.push(Piece::Rook(PieceData::new(Position::new(0, 7), Color::Black), false));
        board.white_pieces.push(Piece::Knight(PieceData::new(Position::new(1, 7), Color::Black)));
        board.white_pieces.push(Piece::Bishop(PieceData::new(Position::new(2, 7), Color::Black)));
        board.white_pieces.push(Piece::Queen(PieceData::new(Position::new(3, 7), Color::Black)));
        board.white_pieces.push(Piece::King(PieceData::new(Position::new(4, 7), Color::Black), false));
        board.white_pieces.push(Piece::Bishop(PieceData::new(Position::new(5, 7), Color::Black)));
        board.white_pieces.push(Piece::Knight(PieceData::new(Position::new(6, 7), Color::Black)));
        board.white_pieces.push(Piece::Rook(PieceData::new(Position::new(7, 7), Color::Black), false));

        board
    }
}

impl fmt::Display for Board {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board: [[String; 8]; 8] = [
            ["□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string()],
            ["■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string()],
            ["□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string()],
            ["■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string()],
            ["□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string()],
            ["■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string()],
            ["□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string()],
            ["■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string()],
        ];

        self.white_pieces.iter().for_each(|p| {
            let Position { file: i, rank: j } = *p.position();
            board[j][i] = p.to_string();
        });
        self.black_pieces.iter().for_each(|p| {
            let Position { file: i, rank: j } = *p.position();
            board[j][i] = p.to_string();
        });

        board.iter().rev().for_each(|rank| {
            let line = rank
                .iter()
                .fold("".to_string(), |acc, square| acc + &square.to_string());
            writeln!(f, "{}", line).unwrap();
        });

        Ok(())
    }
}

fn main() {
    let board = Board::new();

    println!("The game has started.");
    println!("{}", board);

    let mut color = Color::White;

    loop {
        let mut turn = String::new();

        io::stdin()
            .read_line(&mut turn)
            .ok()
            .expect("Failed to read line.");

        turn.pop();

        let tokens = turn.tokenize();

        if let Ok(mut tokens) = tokens {
            validate(&mut tokens);
        }

        color = match color {
            Color::White => {
                println!("White turn {}.", turn);
                Color::Black
            }
            Color::Black => {
                println!("Black turn turn {}.", turn);
                Color::White
            }
        };
    }
}
