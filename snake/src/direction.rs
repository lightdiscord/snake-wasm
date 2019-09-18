#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right
        }
    }

    pub fn is_opposite(self, other: Direction) -> bool {
        self.opposite() == other
    }
}
