use crate::error::CatchAllError;
use crate::position::Position;
use crate::r#move::Direction;
use crate::r#move::Move;

use std::ops;

pub struct Path(Vec<Position>);

impl Default for Path {
    fn default() -> Self {
        Path(Vec::new())
    }
}

impl ops::Deref for Path {
    type Target = Vec<Position>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Path {
    #[rustfmt::skip]
    pub fn new(pos: &Position, mv: &Move) -> Result<Self, CatchAllError> {
        match mv {
            Move::Straight(Direction::Up, steps, _) => Ok(Path::up(&pos, steps.clone())),
            Move::Straight(Direction::Down, steps, _) => Ok(Path::down(&pos, steps.clone())),
            Move::Straight(Direction::Right, steps, _) => Ok(Path::right(&pos, steps.clone())),
            Move::Straight(Direction::Left, steps, _) => Ok(Path::left(&pos, steps.clone())),
            Move::Diagonal(Direction::Up, Direction::Right, steps, _) => Ok(Path::up_right(&pos, steps.clone())),
            Move::Diagonal(Direction::Up, Direction::Left, steps, _) => Ok(Path::up_left(&pos, steps.clone())),
            Move::Diagonal(Direction::Down, Direction::Right, steps, _) => Ok(Path::down_right(&pos, steps.clone())),
            Move::Diagonal(Direction::Down, Direction::Left, steps, _) => Ok(Path::down_left(&pos, steps.clone())),
            Move::Jump(_) => Ok(Path::default()),
            _ => Err(CatchAllError::InvalidPath),
        }
    }

    fn up(pos: &Position, steps: usize) -> Self {
        Path(
            (pos.rank..pos.rank + steps)
                .skip(1)
                .map(move |r| Position::new(pos.file, r))
                .collect(),
        )
    }

    fn down(pos: &Position, steps: usize) -> Self {
        Path(
            (pos.rank - steps..pos.rank)
                .skip(1)
                .rev()
                .map(move |r| Position::new(pos.file, r))
                .collect(),
        )
    }

    fn right(pos: &Position, steps: usize) -> Self {
        Path(
            (pos.file..pos.file + steps)
                .skip(1)
                .map(move |f| Position::new(f, pos.rank))
                .collect(),
        )
    }

    fn left(pos: &Position, steps: usize) -> Self {
        Path(
            (pos.file - steps..pos.file)
                .skip(1)
                .rev()
                .map(move |f| Position::new(f, pos.rank))
                .collect(),
        )
    }

    fn up_right(pos: &Position, steps: usize) -> Self {
        Path(
            (pos.file..pos.file + steps)
                .zip(pos.rank..pos.rank + steps)
                .skip(1)
                .map(move |(f, r)| Position::new(f, r))
                .collect(),
        )
    }

    fn up_left(pos: &Position, steps: usize) -> Self {
        Path(
            (pos.file - steps..=pos.file)
                .rev()
                .zip(pos.rank..pos.rank + steps)
                .skip(1)
                .map(move |(f, r)| Position::new(f, r))
                .collect(),
        )
    }

    fn down_right(pos: &Position, steps: usize) -> Self {
        Path(
            (pos.file..pos.file + steps)
                .zip((pos.rank - steps..=pos.rank).rev())
                .skip(1)
                .map(move |(f, r)| Position::new(f, r))
                .collect(),
        )
    }

    fn down_left(pos: &Position, steps: usize) -> Self {
        Path(
            (pos.file - steps..pos.file)
                .zip(pos.rank - steps..pos.rank)
                .skip(1)
                .rev()
                .map(move |(f, r)| Position::new(f, r))
                .collect(),
        )
    }
}
