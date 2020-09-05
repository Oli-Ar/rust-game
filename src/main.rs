#![windows_subsystem = "windows"]

mod game;
mod structs;

use game::{Game, GameOptions};

fn main() {
    // Initialises the game by creating a game variable using the game struct. The new function for
    // game takes in a GameOptions struct which implements default meaning if no other fields are
    // passed in game options default to: board_size: 7, dice_sides: 6 and random_obstacles: true
    let mut game = Game::new(GameOptions {
        ..Default::default()
    });
    game.render()
}
