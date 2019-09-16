use std::collections::LinkedList;
use rand::prelude::*;

use failure::{ Fail, Fallible };

/// Coords(x, y)
#[derive(Clone, Copy, PartialEq)]
pub struct Coords(usize, usize);

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "invalid coordinates")]
    InvalidCoordinates,

    #[fail(display = "tried to eat himself")]
    TriedToEatHimself
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Direction {
    fn is_opposite(&self, other: Direction) -> bool {
        match self {
            Direction::Up => (other == Direction::Down),
            Direction::Down => (other == Direction::Up),
            Direction::Right => (other == Direction::Left),
            Direction::Left => (other == Direction::Right)
        }
    }
}

impl Coords {
    pub fn new_with_random(rng: &mut ThreadRng, limit: usize) -> Self {
        Coords (
            rng.gen_range(0, limit),
            rng.gen_range(0, limit)
        )
    }

    #[inline]
    pub fn x(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn y(&self) -> usize {
        self.1
    }

    fn try_add(&mut self, direction: Direction, limit: usize) -> Fallible<()> {
        Ok(match direction {
            Direction::Right if self.0 != limit - 1 => {
                self.0 += 1;
            },
            Direction::Down if self.1 != limit - 1 => {
                self.1 += 1;
            },
            Direction::Left if self.0 != 0 => {
                self.0 -= 1;
            },
            Direction::Up if self.1 != 0 => {
                self.1 -= 1;
            },
            _ => {
                return Err(Error::InvalidCoordinates)?;
            }
        })
    }
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Coords>
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

pub struct Game {
    size: usize,
    snake: Snake,
    goal: Coords,
    rng: ThreadRng
}

impl Game {
    pub fn new() -> Self {
        let size = 30;
        let mut rng = ThreadRng::default();
        let start = Coords::new_with_random(&mut rng, size);
        let mut game = Game {
            size,
            snake: Snake::new(start),
            goal: Coords(0, 0),
            rng
        };

        game.replace_goal();
        return game;
    }

    pub fn tick(&mut self) -> Fallible<()> {
        let mut front = *self.snake.body.front().unwrap();
        front.try_add(self.snake.direction, self.size())?;

        self.snake.body.push_front(front);

        if self.goal == front {
            self.replace_goal();
        } else {
            self.snake.body.pop_back();
        }

        Ok(())
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn snake(&self) -> &Snake {
        &self.snake
    }

    #[inline]
    pub fn goal(&self) -> &Coords {
        &self.goal
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if !self.snake.direction.is_opposite(direction) {
            self.snake.direction = direction;
        }
    }

    fn replace_goal(&mut self) {
        self.goal = Coords::new_with_random(&mut self.rng, self.size);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
