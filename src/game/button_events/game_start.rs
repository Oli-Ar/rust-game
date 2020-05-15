use crate::game::Game;
use crate::structs::Player;

impl Game {
  // Changes configs of game when player amount is picked
  pub fn config_game(&mut self, player_number: i32) {
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
        cell: None,
        roll: None
      };
      player_vec.push(player);
    }

    // Sets the high score to first player, all have score of 0
    self.top_player = Some(player_vec[0].clone());
    // Updates players vector to Some containing the vector of players
    self.players = Some(player_vec);
  }
}