extern crate piston_window;
use piston_window::{Button, Key};

//TODO: Clean up the code

#[derive(Debug)]
// Data to be stored in each cell to correctly render the game
pub struct Cell {
  pub cell_number: i32, // Number of the cell on 7*7 board (1-49)
  pub x: i32, // x position on a 7*7 board
  pub y: i32, // y position on a 7*7 board
  pub player: Option<Vec<i16>> // Option to mark if a player is on the board
}

#[derive(Debug)]
// Data to be stored about each player
pub struct Player {
  pub number: i32, // The players number between 1 and 4
  pub score: i32, // The score and therefore cell of the player
  pub cell: Option<Cell> // The cell the player is currently on `None` at start
}

#[derive(Debug)]
// The data to be stored about the game
// TODO: Add obstacles
pub struct Game {
  pub active: bool, // Whether the game has started
  pub player_count: Option<i32>, // Amount of players - none if game not started
  pub players: Option<Vec<Player>>, // The players playing - none if game not started
}

impl Game {
  // Method to check how many players in game based on user input
  pub fn pressed(&mut self, btn: &Button) {
    // Match statement to check if the key is a number key in the range 2-4, assigns the value to player_count
    match btn {
      &Button::Keyboard(Key::D2) => {self.player_count = Some(2); self.active = true;},
      &Button::Keyboard(Key::D3) => {self.player_count = Some(3); self.active = true;},
      &Button::Keyboard(Key::D4) => {self.player_count = Some(4); self.active = true;},
      _ => {self.active = false}
    }

    // Calls a self method to generate a vector of player object to match the amount of players
    self.gen_player_vec();
  }

  // Function to generate a vector of players
  fn gen_player_vec(&mut self) {
    // Defines a vector of players with a capacity equal to player count
    let mut player_vec: Vec<Player> = Vec::with_capacity(self.player_count.unwrap() as usize);
    // Loop to create as many players as needed
    for i in 1..self.player_count.unwrap()+1 {
      let player = Player {
        number: i,
        score: 0,
        cell: None
      };
      player_vec.push(player);
    }
    // Updates players vector to Some containing the vecotr of players
    self.players = Some(player_vec);
  }
}