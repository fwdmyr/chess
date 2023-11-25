use std::collections::HashMap;
use std::fmt;
use std::io;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum CatchAllError {
    InvalidPosition,
    InvalidPath,
    BlockedPath,
    EmptyField,
    UnreachableField,
    BadParse,
}

impl fmt::Display for CatchAllError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CatchAllError::InvalidPosition => write!(f, "the position is invalid"),
            CatchAllError::InvalidPath => write!(f, "the path is invalid"),
            CatchAllError::BlockedPath => write!(f, "the path is blocked"),
            CatchAllError::EmptyField => write!(f, "the field is empty"),
            CatchAllError::UnreachableField => write!(f, "the field is unreachable"),
            CatchAllError::BadParse => write!(f, "could not parse literal"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
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

impl Piece {
    pub fn update(&mut self) {
        match self {
            Piece::Pawn(_, ref mut has_moved)
            | Piece::Rook(_, ref mut has_moved)
            | Piece::King(_, ref mut has_moved) => {
                *has_moved = true;
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

    #[rustfmt::skip]
    pub fn can_reach(&self, from: &Position, to: &Position, is_capture: bool) -> Result<(), CatchAllError> {
        match self {
            Piece::Pawn(color, has_moved) => Piece::can_reach_pawn(from, to, color, *has_moved, is_capture),
            Piece::Knight(_) => Piece::can_reach_knight(from, to),
            Piece::Bishop(_) => Piece::can_reach_bishop(from, to),
            Piece::Rook(_, _) => Piece::can_reach_rook(from, to),
            Piece::Queen(_) => Piece::can_reach_queen(from, to),
            Piece::King(_, has_moved) => Piece::can_reach_king(from, to, *has_moved),
        }
    }

    pub fn is_unobstructed(
        &self,
        pieces: &HashMap<Position, Piece>,
        path: &Vec<Position>,
    ) -> Result<(), CatchAllError> {
        match self {
            Piece::Knight(_) => Ok(()),
            _ => path
                .iter()
                .rev()
                .skip(1)
                .any(|position| pieces.iter().any(|(k, _)| k == position))
                .eq(&false)
                .then(|| ())
                .ok_or(CatchAllError::BlockedPath),
        }
    }

    #[rustfmt::skip]
    fn can_reach_pawn(
        from: &Position, to: &Position, color: &Color, has_moved: bool, is_capture: bool) -> Result<(), CatchAllError> {
        match color {
            Color::White => match from.distance_to(to) {
                Distance { file: 0, rank: 2 } if !has_moved && !is_capture => Ok(()),
                Distance { file: 0, rank: 1 } if !is_capture => Ok(()),
                Distance { file: -1 | 1, rank: 1, } if is_capture => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
            },
            Color::Black => match from.distance_to(to) {
                Distance { file: 0, rank: -2 } if !has_moved && !is_capture => Ok(()),
                Distance { file: 0, rank: -1 } if !is_capture => Ok(()),
                Distance { file: -1 | 1, rank: -1, } if is_capture => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
            },
        }
    }

    #[rustfmt::skip]
    fn can_reach_knight(from: &Position, to: &Position) -> Result<(), CatchAllError> {
        match from.distance_to(to) {
            Distance { file: -1 | 1, rank: -2 | 2 } => Ok(()),
            Distance { file: -2 | 2, rank: -1 | 1 } => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
        }
    }

    #[rustfmt::skip]
    fn can_reach_bishop(from: &Position, to: &Position) -> Result<(), CatchAllError> {
        match from.distance_to(to) {
            Distance { file, rank } if file.abs()==rank.abs() && file != 0 => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
        }
    }

    #[rustfmt::skip]
    fn can_reach_rook(from: &Position, to: &Position) -> Result<(), CatchAllError> {
        match from.distance_to(to) {
            Distance { file: _, rank: 0 } => Ok(()),
            Distance { file: 0, rank: _ } => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
        }
    }

    #[rustfmt::skip]
    fn can_reach_queen(from: &Position, to: &Position) -> Result<(), CatchAllError> {
        match from.distance_to(to) {
            Distance { file, rank } if file.abs()==rank.abs() && file!=0 => Ok(()),
            Distance { file: _, rank: 0 } => Ok(()),
            Distance { file: 0, rank: _ } => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
        }
    }

    #[rustfmt::skip]
    fn can_reach_king(from: &Position, to: &Position, has_moved: bool) -> Result<(), CatchAllError> {
        match from.distance_to(to) {
            Distance { file: -1 | 0 | 1, rank: -1 | 0 | 1 } => Ok(()),
            Distance { file: -3 | 2, rank: 0 } if !has_moved => Ok(()),
            _ => Err(CatchAllError::UnreachableField),
        }
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self {
            Piece::Pawn(color, _) => match color {
                Color::White => "♟".to_string(),
                Color::Black => "♙".to_string(),
            },
            Piece::Knight(color) => match color {
                Color::White => "♞".to_string(),
                Color::Black => "♘".to_string(),
            },
            Piece::Bishop(color) => match color {
                Color::White => "♝".to_string(),
                Color::Black => "♗".to_string(),
            },
            Piece::Rook(color, _) => match color {
                Color::White => "♜".to_string(),
                Color::Black => "♖".to_string(),
            },
            Piece::Queen(color) => match color {
                Color::White => "♛".to_string(),
                Color::Black => "♕".to_string(),
            },
            Piece::King(color, _) => match color {
                Color::White => "♚".to_string(),
                Color::Black => "♔".to_string(),
            },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Position {
    file: usize,
    rank: usize,
}

#[derive(Debug, PartialEq)]
pub struct Distance {
    pub file: isize,
    pub rank: isize,
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

    fn distance_to(&self, other: &Position) -> Distance {
        Distance {
            file: other.file as isize - self.file as isize,
            rank: other.rank as isize - self.rank as isize,
        }
    }

    #[rustfmt::skip]
    fn path_to(&self, other: &Position) -> Result<Vec<Position>, CatchAllError> {
        let from = self.clone();
        let to = other.clone();
        todo!("Find path [from ,to). The last field will be checked for captures.");
        match from.distance_to(&to) {
            Distance { file: 0, rank: 1.. } => Ok(Position::file_path_fwd(from..=to)),
            Distance { file: 0, rank: ..=-1 } => Ok(Position::file_path_rev(from..=to)),
            Distance { file: 1.., rank: 0 } => Ok(Position::rank_path_fwd(from..=to)),
            Distance { file: ..=-1, rank: 0 } => Ok(Position::rank_path_rev(from..=to)),
            Distance { file: f, rank: r } if f.abs() == r.abs() && f > 0 && r > 0 => Ok(Position::diagonal_path_fwd_fwd(from..=to)),
            Distance { file: f, rank: r } if f.abs() == r.abs() && f > 0 && r < 0 => Ok(Position::diagonal_path_fwd_rev(from..=to)),
            Distance { file: f, rank: r } if f.abs() == r.abs() && f < 0 && r > 0 => Ok(Position::diagonal_path_rev_fwd(from..=to)),
            Distance { file: f, rank: r } if f.abs() == r.abs() && f < 0 && r < 0 => Ok(Position::diagonal_path_rev_rev(from..=to)),
            Distance { file: -2 | 2, rank: -1 | 1 } => Ok(Vec::new()),
            Distance { file: -1 | 1, rank: -2 | 2 } => Ok(Vec::new()),
            _ => Err(CatchAllError::InvalidPath)
        }
    }

    fn file_path_fwd(range: RangeInclusive<Position>) -> Vec<Position> {
        (range.start().file..=range.end().file)
            .flat_map(move |x| {
                (range.start().rank..=range.end().rank)
                    .skip(1)
                    .map(move |y| Position::new(x, y))
            })
            .collect()
    }

    fn file_path_rev(range: RangeInclusive<Position>) -> Vec<Position> {
        (range.start().file..=range.end().file)
            .flat_map(move |x| {
                (range.end().rank..range.start().rank).map(move |y| Position::new(x, y))
            })
            .collect()
    }

    fn rank_path_fwd(range: RangeInclusive<Position>) -> Vec<Position> {
        (range.start().rank..=range.end().rank)
            .flat_map(move |y| {
                (range.start().file..=range.end().file)
                    .skip(1)
                    .map(move |x| Position::new(x, y))
            })
            .collect()
    }

    fn rank_path_rev(range: RangeInclusive<Position>) -> Vec<Position> {
        (range.start().rank..=range.end().file)
            .flat_map(move |y| {
                (range.end().file..range.start().file).map(move |x| Position::new(x, y))
            })
            .collect()
    }

    fn diagonal_path_fwd_fwd(range: RangeInclusive<Position>) -> Vec<Position> {
        (range.start().file..=range.end().file)
            .skip(1)
            .map(move |x| Position::new(x, x))
            .collect()
    }

    fn diagonal_path_fwd_rev(range: RangeInclusive<Position>) -> Vec<Position> {
        let anchor_point = range.start().rank;
        let v = (range.start().file..=range.end().file)
            .skip(1)
            .enumerate()
            .map(move |(i, x)| Position::new(x, anchor_point - i))
            .collect();
        println!("{:?}", v);
        v
    }

    fn diagonal_path_rev_fwd(range: RangeInclusive<Position>) -> Vec<Position> {
        let anchor_point = range.end().rank;
        let v = (range.end().file..=range.start().file)
            .skip(1)
            .enumerate()
            .map(move |(i, x)| Position::new(x, anchor_point - i))
            .collect();
        println!("{:?}", v);
        v
    }

    fn diagonal_path_rev_rev(range: RangeInclusive<Position>) -> Vec<Position> {
        (range.end().file..=range.start().file)
            .skip(1)
            .map(move |x| Position::new(x, x))
            .collect()
    }
}

impl FromStr for Position {
    type Err = CatchAllError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let file = s[0..1]
            .to_lowercase()
            .parse::<char>()
            .map_err(|_| CatchAllError::BadParse)?;
        let rank = s[1..2]
            .parse::<usize>()
            .map_err(|_| CatchAllError::BadParse)?;

        Ok(Position::new(file as usize - 97, rank - 1))
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
    pieces: HashMap<Position, Piece>,
}

impl Board {
    #[rustfmt::skip]
    fn new() -> Self {
        let mut board = Self {
            pieces: HashMap::new(),
        };

        board.pieces.insert(Position::new(0, 0), Piece::Rook( Color::White, false));
        board.pieces.insert(Position::new(1, 0), Piece::Knight( Color::White));
        board.pieces.insert(Position::new(2, 0), Piece::Bishop( Color::White));
        board.pieces.insert(Position::new(3, 0), Piece::Queen( Color::White));
        board.pieces.insert(Position::new(4, 0), Piece::King( Color::White, false));
        board.pieces.insert(Position::new(5, 0), Piece::Bishop( Color::White));
        board.pieces.insert(Position::new(6, 0), Piece::Knight( Color::White));
        board.pieces.insert(Position::new(7, 0), Piece::Rook( Color::White, false));
        board.pieces.insert(Position::new(0, 1), Piece::Pawn( Color::White, false));
        board.pieces.insert(Position::new(1, 1), Piece::Pawn( Color::White, false));
        board.pieces.insert(Position::new(2, 1), Piece::Pawn( Color::White, false));
        board.pieces.insert(Position::new(3, 1), Piece::Pawn( Color::White, false));
        board.pieces.insert(Position::new(4, 1), Piece::Pawn( Color::White, false));
        board.pieces.insert(Position::new(5, 1), Piece::Pawn( Color::White, false));
        board.pieces.insert(Position::new(6, 1), Piece::Pawn( Color::White, false));
        board.pieces.insert(Position::new(7, 1), Piece::Pawn( Color::White, false));
        board.pieces.insert(Position::new(0, 6), Piece::Pawn( Color::Black, false));
        board.pieces.insert(Position::new(1, 6), Piece::Pawn( Color::Black, false));
        board.pieces.insert(Position::new(2, 6), Piece::Pawn( Color::Black, false));
        board.pieces.insert(Position::new(3, 6), Piece::Pawn( Color::Black, false));
        board.pieces.insert(Position::new(4, 6), Piece::Pawn( Color::Black, false));
        board.pieces.insert(Position::new(5, 6), Piece::Pawn( Color::Black, false));
        board.pieces.insert(Position::new(6, 6), Piece::Pawn( Color::Black, false));
        board.pieces.insert(Position::new(7, 6), Piece::Pawn( Color::Black, false));
        board.pieces.insert(Position::new(0, 7), Piece::Rook( Color::Black, false));
        board.pieces.insert(Position::new(1, 7), Piece::Knight( Color::Black));
        board.pieces.insert(Position::new(2, 7), Piece::Bishop( Color::Black));
        board.pieces.insert(Position::new(3, 7), Piece::Queen( Color::Black));
        board.pieces.insert(Position::new(4, 7), Piece::King( Color::Black, false));
        board.pieces.insert(Position::new(5, 7), Piece::Bishop( Color::Black));
        board.pieces.insert(Position::new(6, 7), Piece::Knight( Color::Black));
        board.pieces.insert(Position::new(7, 7), Piece::Rook( Color::Black, false));

        board
    }

    pub fn advance(
        &mut self,
        color: &Color,
        from: &Position,
        to: &Position,
    ) -> Result<(), CatchAllError> {
        {
            // Check if piece of correct color is at from position.
            let piece = self
                .pieces
                .get(from)
                .map_or(Err(CatchAllError::EmptyField), |p| {
                    if &p.color() == color {
                        Ok(p)
                    } else {
                        Err(CatchAllError::EmptyField)
                    }
                })?;

            // Check if piece is at to.
            // If piece of opposite color, the action will be capture.
            // If piece of same color, the path is blocked.
            let is_capture = self.pieces.get(to).map_or(Ok(false), |p| {
                if &p.color() == color {
                    Err(CatchAllError::BlockedPath)
                } else {
                    Ok(true)
                }
            })?;

            // Check if piece can reach the to position from the from position.
            piece.can_reach(from, to, is_capture)?;

            // Construct the path the piece can take from to.
            let path = from.path_to(to)?;

            // Check if the path from to is unobstructed.
            piece.is_unobstructed(&self.pieces, &path)?;
        }

        // Move the piece from to.
        let piece = self.pieces.remove(from).ok_or(CatchAllError::EmptyField)?;
        self.pieces.insert(to.clone(), piece);
        self.pieces.entry(to.clone()).and_modify(|v| v.update());

        Ok(())
    }
}

impl fmt::Display for Board {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board: [[String; 9]; 9] = [
            [" ".to_string(), "A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string(), "G".to_string(), "H".to_string()],
            ["1".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string()],
            ["2".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string()],
            ["3".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string()],
            ["4".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string()],
            ["5".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string()],
            ["6".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string()],
            ["7".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string()],
            ["8".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string(), "■".to_string(), "□".to_string()],
        ];

        self.pieces.iter().for_each(|(k, v)| {
            let Position { file: i, rank: j } = k;
            board[*j+1][*i+1] = v.to_string();
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
    let mut board = Board::new();

    println!("The game has started.");
    println!("{}", board);

    let mut color = Color::White;

    loop {
        match color {
            Color::White => println!("White to play."),
            Color::Black => println!("Black to play."),
        }

        let mut turn = String::new();

        println!("Enter from position.");

        io::stdin()
            .read_line(&mut turn)
            .ok()
            .expect("Failed to read line.");

        let from = Position::from_str(&turn[..]).unwrap();

        println!("Enter to position.");

        let mut turn = String::new();

        io::stdin()
            .read_line(&mut turn)
            .ok()
            .expect("Failed to read line.");

        let to = Position::from_str(&turn[..]).unwrap();

        let res = board.advance(&color, &from, &to);

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

        println!("{}", board);
    }
}
