use failure::Fail;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "invalid coordinates")]
    InvalidCoordinates,

    #[fail(display = "tried to eat himself")]
    TriedToEatHimself,

    #[fail(display = "tried to tick in a no playable state")]
    TickWhileNotPlaying
}
