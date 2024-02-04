#![warn(clippy::all, rust_2018_idioms)]

pub use app::TemplateApp;
pub use buttons::*;
pub use windows::*;

mod windows;

mod app;
mod buttons;
