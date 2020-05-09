extern crate piston_window;

use piston_window::{Button, Key};
use crate::structs::Player;
use crate::game::Game;

impl Game {
  // Method to check how many players in game based on user input
  pub fn pressed(&mut self, btn: &Button) {
    // Match statement to check if the key is a number key in the range 2-4, runs function to config game
    match btn {
      &Button::Keyboard(Key::D2) => { self.config_game(2) },
      &Button::Keyboard(Key::D3) => { self.config_game(3) },
      &Button::Keyboard(Key::D4) => { self.config_game(4) },
      _ => {}
    }
  }

  // Changes configs of game when player amount is picked
  fn config_game(&mut self, player_number: i32) {
    // Sets player count and changes game to active
    self.player_count = Some(player_number);
    self.active = true;

    // Uses self method to generate vector of players with correct number of players
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
    // Updates players vector to Some containing the vector of players
    self.players = Some(player_vec);
  }
}