pub mod dark {
    pub const PRIMARY: &'static str = "#ffffff";
    pub const SECONDARY: &'static str = "#000000";
    pub const GRID: &'static str = "#7f7f7f";
    pub const GOAL: &'static str = "#d50000";
}

pub mod light {
    pub const PRIMARY: &'static str = "#000000";
    pub const SECONDARY: &'static str = "#ffffff";
    pub const GRID: &'static str = "#7f7f7f";
    pub const GOAL: &'static str = "#d50000";
}

pub use self::dark as current;
