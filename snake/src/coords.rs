use super::Error;
use rand::prelude::*;
use super::Direction;
use failure::Fallible;

#[derive(Clone, Copy, PartialEq)]
pub struct Coords {
    pub x: usize,
    pub y: usize
}

impl Coords {
    #[inline]
    pub fn new(x: usize, y: usize) -> Self {
        Coords {
            x,
            y
        }
    }

    #[inline]
    pub fn new_with_random(rng: &mut ThreadRng, limit: usize) -> Self {
        Coords::new(rng.gen_range(0, limit), rng.gen_range(0, limit))
    }

    pub(crate) fn try_add(&mut self, direction: Direction, limit: usize) -> Fallible<()> {
        Ok(match direction {
            Direction::Right if self.x != limit - 1 => (self.x += 1),
            Direction::Down if self.y != limit - 1 => (self.y += 1),
            Direction::Left if self.x != 0 => (self.x -= 1),
            Direction::Up if self.y != 0 => (self.y -= 1),
            _ => {
                return Err(Error::InvalidCoordinates)?;
            }
        })
    }
}

