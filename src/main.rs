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
    todo!("Ensure that the king cannot castle through check.");
    todo!("Correctly wind back moves by also restoring piece state (maybe use counter that ticks up on piece move rather than state enum).");
    todo!("Determine checkmate and stalemate (brute-force check all reachable fields for all pieces of king's color).");
    todo!("Resolve en-passant.");
    todo!("Handle promotion.");
    todo!("Reduce responsibilities of board");
    todo!("Handle draw by three-fold repetition (maybe hash the board state), by insufficient material, by 50 move rule (simple capture counter)");

    Gui::run(Settings {
        window: window::Settings {
            size: (800, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
