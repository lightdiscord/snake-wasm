pub mod coords;
pub mod direction;
pub mod game;
pub mod snake;
pub mod error;

pub use coords::Coords;
pub use direction::Direction;
pub use game::Game;
pub use snake::Snake;
pub use error::Error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
