pub struct Distance {
    pub file: isize,
    pub rank: isize,
}

impl Distance {
    pub fn new(from: &Position, to: &Position) -> Self {
        Self {
            file: to.file as isize - from.file as isize,
            rank: to.rank as isize - from.rank as isize,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Position {
    pub file: usize,
    pub rank: usize,
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

    fn valid(&self) -> bool {
        (self.file < 8) && (self.rank < 8)
    }
}
