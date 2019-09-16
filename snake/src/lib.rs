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
    TriedToEatHimself,

    #[fail(display = "tried to tick but game end")]
    TickAfterEnd
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
    #[inline]
    pub fn new_with_random(rng: &mut ThreadRng, limit: usize) -> Self {
        Coords (rng.gen_range(0, limit), rng.gen_range(0, limit))
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
            Direction::Right if self.0 != limit - 1 => (self.0 += 1),
            Direction::Down if self.1 != limit - 1 => (self.1 += 1),
            Direction::Left if self.0 != 0 => (self.0 -= 1),
            Direction::Up if self.1 != 0 => (self.1 -= 1),
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
    end: bool,
    goal: Coords,
    rng: ThreadRng
}

impl Game {
    pub fn new() -> Self {
        let size = 10;
        let mut rng = ThreadRng::default();
        let mut start = Coords::new_with_random(&mut rng, size / 2);
        start.1 += size / 2;
        let mut game = Game {
            size,
            snake: Snake::new(start),
            goal: Coords(0, 0),
            end: false,
            rng
        };

        game.replace_goal();
        return game;
    }

    fn inner_tick(&mut self) -> Fallible<()> {
        let mut front = *self.snake.body.front().unwrap();
        front.try_add(self.snake.direction, self.size())?;

        if self.is_snake(front) {
            return Err(Error::TriedToEatHimself)?;
        }

        self.snake.body.push_front(front);

        if self.goal == front {
            self.replace_goal();
        } else {
            self.snake.body.pop_back();
        }

        Ok(())
    }

    pub fn tick(&mut self) -> Fallible<()> {
        if self.end {
            return Err(Error::TickAfterEnd)?;
        }

        let result = self.inner_tick();
        if result.is_err() {
            self.end = true;
        }

        return result;
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

    #[inline]
    pub fn end(&self) -> bool {
        self.end
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if !self.snake.direction.is_opposite(direction) {
            self.snake.direction = direction;
        }
    }

    fn is_snake(&self, coords: Coords) -> bool {
        self.snake.body.iter().any(|&x| x == coords)
    }

    fn replace_goal(&mut self) {
        self.goal = Coords::new_with_random(&mut self.rng, self.size);
        while self.is_snake(self.goal) {
            self.goal = Coords::new_with_random(&mut self.rng, self.size);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
