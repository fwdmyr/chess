use std::collections::HashMap;
use std::fmt;
use std::io;
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
    fn path(&self, mv: &Move) -> Result<Vec<Self>, CatchAllError> {
        match mv {
            Move::Straight(Direction::Up, steps, _) => Ok(self.path_up(steps.clone())),
            Move::Straight(Direction::Down, steps, _) => Ok(self.path_down(steps.clone())),
            Move::Straight(Direction::Right, steps, _) => Ok(self.path_right(steps.clone())),
            Move::Straight(Direction::Left, steps, _) => Ok(self.path_left(steps.clone())),
            Move::Diagonal(Direction::Up, Direction::Right, steps, _) => Ok(self.path_up_right(steps.clone())),
            Move::Diagonal(Direction::Up, Direction::Left, steps, _) => Ok(self.path_up_left(steps.clone())),
            Move::Diagonal(Direction::Down, Direction::Right, steps, _) => Ok(self.path_down_right(steps.clone())),
            Move::Diagonal(Direction::Down, Direction::Left, steps, _) => Ok(self.path_down_left(steps.clone())),
            Move::Jump(_) => Ok(Vec::new()),
            _ => Err(CatchAllError::InvalidPath),
        }
    }

    fn path_up(&self, steps: usize) -> Vec<Self> {
        (self.rank..self.rank + steps)
            .skip(1)
            .map(move |r| Position::new(self.file, r))
            .collect()
    }

    fn path_down(&self, steps: usize) -> Vec<Self> {
        (self.rank - steps..self.rank)
            .rev()
            .skip(1)
            .map(move |r| Position::new(self.file, r))
            .collect()
    }

    fn path_right(&self, steps: usize) -> Vec<Self> {
        (self.file..self.file + steps)
            .skip(1)
            .map(move |f| Position::new(f, self.rank))
            .collect()
    }

    fn path_left(&self, steps: usize) -> Vec<Self> {
        (self.file - steps..self.file)
            .rev()
            .skip(1)
            .map(move |f| Position::new(f, self.rank))
            .collect()
    }

    fn path_up_right(&self, steps: usize) -> Vec<Self> {
        (self.file..self.file + steps)
            .zip(self.rank..self.rank + steps)
            .skip(1)
            .map(move |(f, r)| Position::new(f, r))
            .collect()
    }

    fn path_up_left(&self, steps: usize) -> Vec<Self> {
        (self.file - steps..self.file)
            .rev()
            .zip(self.rank..self.rank + steps)
            .skip(1)
            .map(move |(f, r)| Position::new(f, r))
            .collect()
    }

    fn path_down_right(&self, steps: usize) -> Vec<Self> {
        (self.file..self.file + steps)
            .zip((self.rank - steps..self.rank).rev())
            .skip(1)
            .map(move |(f, r)| Position::new(f, r))
            .collect()
    }

    fn path_down_left(&self, steps: usize) -> Vec<Self> {
        (self.file - steps..self.file)
            .zip(self.rank - steps..self.rank)
            .rev()
            .skip(1)
            .map(move |(f, r)| Position::new(f, r))
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

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub enum Action {
    Regular,
    Capture,
}

#[derive(Debug)]
pub enum Move {
    Straight(Direction, usize, Action),
    Diagonal(Direction, Direction, usize, Action),
    Jump(Action),
    Invalid,
}

impl Move {
    #[rustfmt::skip]
    fn new(from: &Position, to: &Position, action: Action) -> Self {
        match from.distance_to(to) {
            Distance { file: 0, rank: r @1.. } => Move::Straight(Direction::Up, r as usize, action),
            Distance { file: 0, rank: r @..=-1 } => Move::Straight(Direction::Down, r.abs() as usize, action),
            Distance { file: 1.., rank: r @0 } => Move::Straight(Direction::Right, r as usize, action),
            Distance { file: ..=-1, rank: r @0 } => Move::Straight(Direction::Left, r.abs() as usize, action),
            Distance { file: f @1.., rank: r @1.. } if f == r=> Move::Diagonal(Direction::Up, Direction::Right, r as usize, action),
            Distance { file: f @1.., rank: r @..=-1 } if f == -r=> Move::Diagonal(Direction::Down, Direction::Right, r.abs() as usize, action),
            Distance { file: f @..=-1, rank: r @1.. } if f == -r=> Move::Diagonal(Direction::Up, Direction::Left, r as usize, action),
            Distance { file: f @..=-1, rank: r @..=-1 } if f == r=> Move::Diagonal(Direction::Down, Direction::Left, r.abs() as usize, action),
            Distance { file: -2 | 2, rank: -1 | 1 } => Move::Jump(action),
            Distance { file: -1 | 1, rank: -2 | 2 } => Move::Jump(action),
            _ => Move::Invalid,
        }
    }
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

    fn assess_move(&self, pos: &Position, mv: &Move) -> Result<(), CatchAllError> {
        pos.path(mv)?
            .iter()
            .try_fold((), |_, position| self.has_piece(position))
    }

    fn update(&mut self, from: &Position, to: &Position) -> Result<(), CatchAllError> {
        let mut piece = self.pieces.remove(from).ok_or(CatchAllError::EmptyField)?;
        piece.update();
        self.pieces.insert(to.clone(), piece);

        Ok(())
    }

    #[rustfmt::skip]
    fn assess_turn(&self, color: &Color, from: &Position, to: &Position) -> Result<(), CatchAllError> {
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

        Ok(())
    }

    #[rustfmt::skip]
    pub fn advance(&mut self, color: &Color, from: &Position, to: &Position) -> Result<(), CatchAllError> {
        self.assess_turn(color, from, to)?;
        self.update(from, to)?;

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

        println!("Enter turn.");

        io::stdin()
            .read_line(&mut turn)
            .ok()
            .expect("Failed to read line.");

        let from = Position::from_str(&turn[0..2]).unwrap();
        let to = Position::from_str(&turn[2..4]).unwrap();

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
