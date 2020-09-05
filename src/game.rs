mod button_events;
mod fetch_messages;
mod gen_cells_vec;
mod gen_obstacles;
mod render;

use crate::structs::{Cell, Player};
use std::collections::HashMap;

#[derive(Debug)]
// The data to be stored about the game
pub struct Game {
    pub cells: Vec<Cell>,                  // The cells in the game
    pub game_options: GameOptions,         // Small tweaks to change some part of the game
    pub active: bool,                      // Whether the game has started
    pub player_count: Option<i32>,         // Amount of players - none if game not started
    pub players: Option<Vec<Player>>,      // The players playing - none if game not started
    pub top_player: Option<Player>,        // The current leader
    pub messages: HashMap<String, String>, // The externally fetches messages to be displayed in the game
}

impl Game {
    // Static method to create a Game variable with default values
    pub fn new(game_options: GameOptions) -> Self {
        Self {
            cells: self::gen_cells_vec::make_cells_vec(&game_options),
            game_options,
            active: false,
            player_count: None,
            players: None,
            top_player: None,
            messages: self::fetch_messages::fetch_messages(),
        }
    }
}

#[derive(Debug)]
// Small tweaks to the game that can easily be passed as an argument by the user
pub struct GameOptions {
    pub board_size: i32,        // Size of the board
    pub dice_sides: i32,        // How many sides the dice has
    pub random_obstacles: bool, // Whether or not obstacles should be randomly generated
}

// Defaults for game options
impl Default for GameOptions {
    fn default() -> Self {
        Self {
            board_size: 7,
            dice_sides: 6,
            random_obstacles: true,
        }
    }
}
