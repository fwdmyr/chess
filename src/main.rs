mod board;
mod error;
mod gui;
mod r#move;
mod piece;
mod position;
mod game;

use gui::Gui;
use iced::window;
use iced::Sandbox;
use iced::Settings;

pub fn main() -> iced::Result {
    Gui::run(Settings {
        window: window::Settings {
            size: (800, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
