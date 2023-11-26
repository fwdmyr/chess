mod board;
mod error;
mod gui;
mod r#move;
mod piece;
mod position;

use gui::Gui;
use iced::Sandbox;
use iced::Settings;

pub fn main() -> iced::Result {
    Gui::run(Settings::default())
}
