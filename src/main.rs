mod game;
mod structs;

use game::Game;

fn main() {
  // Initialises the game by creating a game variable using the game struct
  let mut game = Game::new(7);
  game.render()
}