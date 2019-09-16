use std::collections::LinkedList;
use super::{ Direction, Coords };

pub struct Snake {
    pub(crate) direction: Direction,
    pub(crate) body: LinkedList<Coords>
}

impl Snake {
    pub fn new(coords: Coords) -> Self {
        let mut body = LinkedList::new();
        body.push_front(coords);

        Snake {
            direction: Direction::Up,
            body
        }
    }

    #[inline]
    pub fn body(&self) -> &LinkedList<Coords> {
        &self.body
    }
}

