// TODO: Setup a way to switch between themes

mod common {
    pub const GRID: &'static str = "#7f7f7f";
    pub const GOAL: &'static str = "#d50000";
    pub const ERROR_BACKGROUND: &'static str = "#c62828";
}

pub mod dark {
    pub use super::common::*;

    pub const PRIMARY: &'static str = "#ffffff";
    pub const SECONDARY: &'static str = "#000000";
}

pub mod light {
    pub use super::common::*;

    pub const PRIMARY: &'static str = "#000000";
    pub const SECONDARY: &'static str = "#ffffff";
}

pub use self::dark as current;
