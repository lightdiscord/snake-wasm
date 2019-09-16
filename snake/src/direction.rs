#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Direction {
    pub fn is_opposite(&self, other: Direction) -> bool {
        match self {
            Direction::Up => (other == Direction::Down),
            Direction::Down => (other == Direction::Up),
            Direction::Right => (other == Direction::Left),
            Direction::Left => (other == Direction::Right)
        }
    }
}
