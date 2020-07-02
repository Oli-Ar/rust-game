mod render;
mod button_events;
mod gen_cells_vec;

use crate::structs::{ Cell, Player };

#[derive(Debug)]
// The data to be stored about the game
// TODO: Add obstacles
pub struct Game {
  pub cells: Vec<Cell>, // The cells in the game
  pub board_size: i32, // Length of one side of the board
  pub active: bool, // Whether the game has started
  pub player_count: Option<i32>, // Amount of players - none if game not started
  pub players: Option<Vec<Player>>, // The players playing - none if game not started
  pub top_player: Option<Player>, // The current leader
}

impl Game {
  // Static method to create a Game variable with default values
  pub fn new(board_size: i32) -> Self {
    Self {
      cells: self::gen_cells_vec::make_cells_vec(board_size),
      board_size,
      active: false,
      player_count: None,
      players: None,
      top_player: None,
    }
  }
}