use rand::prelude::*;
use super::{ Snake, Coords, Error, Direction };
use failure::Fallible;

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
        start.y += size / 2;
        let mut game = Game {
            size,
            snake: Snake::new(start),
            goal: Coords::new(0, 0),
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
