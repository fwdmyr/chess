use crate::error::CatchAllError;
use crate::r#move::Direction;
use crate::r#move::Move;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Position {
    file: usize,
    rank: usize,
}

#[derive(Debug, PartialEq)]
pub struct Distance {
    pub file: isize,
    pub rank: isize,
}

impl Default for Position {
    fn default() -> Self {
        Self { file: 0, rank: 0 }
    }
}

impl Position {
    pub fn new(file: usize, rank: usize) -> Self {
        let position = Self { file, rank };
        assert!(position.valid(), "Invalid position ({}, {})", file, rank);
        position
    }

    pub fn file(&self) -> usize {
        self.file
    }

    pub fn rank(&self) -> usize {
        self.rank
    }

    fn valid(&self) -> bool {
        (self.file < 8) && (self.rank < 8)
    }

    pub fn distance_to(&self, other: &Position) -> Distance {
        Distance {
            file: other.file as isize - self.file as isize,
            rank: other.rank as isize - self.rank as isize,
        }
    }

    #[rustfmt::skip]
    pub fn path(&self, mv: &Move) -> Result<Vec<Self>, CatchAllError> {
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
            .skip(1)
            .rev()
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
            .skip(1)
            .rev()
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
        (self.file - steps..=self.file)
            .rev()
            .zip(self.rank..self.rank + steps)
            .skip(1)
            .map(move |(f, r)| Position::new(f, r))
            .collect()
    }

    fn path_down_right(&self, steps: usize) -> Vec<Self> {
        (self.file..self.file + steps)
            .zip((self.rank - steps..=self.rank).rev())
            .skip(1)
            .map(move |(f, r)| Position::new(f, r))
            .collect()
    }

    fn path_down_left(&self, steps: usize) -> Vec<Self> {
        (self.file - steps..self.file)
            .zip(self.rank - steps..self.rank)
            .skip(1)
            .rev()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r#move::Action;

    #[test]
    fn diagonal_up_left() {
        let position = Position::new(4, 4);
        let mv = Move::Diagonal(Direction::Up, Direction::Left, 3, Action::Regular);
        let path = position.path(&mv);
        let expected = vec![Position::new(3, 5), Position::new(2, 6)];

        // println!("UpLeft");
        // println!("Is:       {:?}", path);
        // println!("Expected: {:?}", expected);

        assert_eq!(path.unwrap(), expected);
    }

    #[test]
    fn diagonal_down_right() {
        let position = Position::new(4, 4);
        let mv = Move::Diagonal(Direction::Down, Direction::Right, 3, Action::Regular);
        let path = position.path(&mv);
        let expected = vec![Position::new(5, 3), Position::new(6, 2)];

        // println!("DownRight");
        // println!("Is:       {:?}", path);
        // println!("Expected: {:?}", expected);

        assert_eq!(path.unwrap(), expected);
    }
}
