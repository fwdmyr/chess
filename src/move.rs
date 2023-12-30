use crate::position::{Distance, Position};

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
    pub fn new(from: &Position, to: &Position, action: Action) -> Self {
        match Distance::new(&from, &to) {
            Distance { file: 0, rank: r @1.. } => Move::Straight(Direction::Up, r as usize, action),
            Distance { file: 0, rank: r @..=-1 } => Move::Straight(Direction::Down, r.abs() as usize, action),
            Distance { file: f @1.., rank: 0 } => Move::Straight(Direction::Right, f as usize, action),
            Distance { file: f @..=-1, rank: 0 } => Move::Straight(Direction::Left, f.abs() as usize, action),
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
