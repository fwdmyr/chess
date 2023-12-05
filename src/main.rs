mod board;
mod error;
mod game;
mod gui;
mod r#move;
mod piece;
mod position;

use gui::Gui;
use iced::window;
use iced::Sandbox;
use iced::Settings;

pub fn main() -> iced::Result {
    todo!("Display pop-up on checkmate or stalemate before resetting game");
    todo!("Reduce responsibilities of board");
    todo!("Handle draw by three-fold repetition (Zobrist hasing), by insufficient material, by 50 move rule (simple capture counter)");

    Gui::run(Settings {
        window: window::Settings {
            size: (800, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
