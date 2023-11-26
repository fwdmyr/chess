mod board;
mod error;
mod game;
mod r#move;
mod piece;
mod position;

use game::Game;

fn main() {
    let mut game = Game::new();
    let _ = game.run();
}
