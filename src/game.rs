mod render;
mod change_state;
mod gen_cells_vec;
mod quick_sort_cells;

use crate::structs::{ Cell, Player };

#[derive(Debug)]
// The data to be stored about the game
// TODO: Add obstacles
pub struct Game {
  pub cells: Vec<Cell>, // The cells in the game
  pub active: bool, // Whether the game has started
  pub player_count: Option<i32>, // Amount of players - none if game not started
  pub players: Option<Vec<Player>>, // The players playing - none if game not started
}

impl Game {
  // Static method to create a Game variable with default values
  pub fn new() -> Self {
    Self {
      cells: self::gen_cells_vec::make_cells_vec(),
      active: false,
      player_count: None,
      players: None
    }
  }
}