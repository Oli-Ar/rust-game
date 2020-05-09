mod game;
mod structs;

use std::{ thread, thread::JoinHandle };
use game::Game;

fn main() {
  // Initialises the game by creating a game variable using the game struct
  let mut game = Game::new();
  game.quick_sort(0, (game.cells.len()-1) as i64);

  // Creates new vec to store opened threads
  let mut open_threads: Vec<JoinHandle<()>> = Vec::new();

  // Uses the render board module to render the game board
  open_threads.push(thread::spawn(move|| {
    game.render()
  }));
  
  #[allow(unused_must_use)]
  // Closes open threads once they have ran
  for thread in open_threads {
    thread.join();
  };
}