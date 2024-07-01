/// Taken from https://github.com/SorenHolstHansen/phosphor-leptos/
/// For performance reason, because compile time are horrendous with the full library
pub mod bug;
pub mod calendar_check;
pub mod calendar_x;
pub mod caret_double_right;
pub mod download;
pub mod gitlab_logo;
pub mod house;
pub mod plus_circle;
pub mod sun;
pub mod sun_horizon;
pub mod warning_circle;
pub mod x;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconWeight {
    Fill,
    Duotone,
    Thin,
    Bold,
    Light,
    Regular,
}
