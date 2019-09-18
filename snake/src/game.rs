use rand::prelude::*;
use super::{ Snake, Coords, Error };
use failure::Fallible;

#[derive(PartialEq)]
pub enum State {
    Playing,
    Dead
}

pub struct Game {
    pub snake: Snake,
    state: State,
    rng: ThreadRng,
    goal: Coords,
    size: usize
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
            state: State::Playing,
            rng
        };

        game.replace_goal();
        return game;
    }

    fn inner_tick(&mut self) -> Fallible<()> {
        let size = self.size();
        let mut front = *self.snake.body.front_mut().unwrap();
        front = front.next(self.snake.direction, size)?;

        if self.snake.is_snake(front) {
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
        if self.state == State::Dead {
            return Err(Error::TickWhileNotPlaying)?;
        }

        let result = self.inner_tick();
        if result.is_err() {
            self.state = State::Dead;
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
    pub fn state(&self) -> &State {
        &self.state
    }

    fn replace_goal(&mut self) {
        self.goal = Coords::new_with_random(&mut self.rng, self.size);
        while self.snake.is_snake(self.goal) {
            self.goal = Coords::new_with_random(&mut self.rng, self.size);
        }
    }
}
