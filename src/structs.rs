#[derive(Debug)]
// Data to be stored in each cell to correctly render the game
pub struct Cell {
  pub cell_number: i16, // Number of the cell on 7*7 board (1-49)
  pub x: i16, // x position on a 7*7 board
  pub y: i16, // y position on a 7*7 board
  pub player: Option<Vec<i16>> // Option to mark if a player is on the board
}

#[allow(dead_code)]
// Data to be stored about each player
pub struct Player {
  pub number: i16, // The players number between 1 and 4
  pub score: i16, // The score and therefore cell of the player
  pub cell: Option<Cell> // The cell the player is currently on `None` at start
}

#[allow(dead_code)]
// The data to be stored about the game
// TODO: Add obstacles
pub struct Game {
  pub players: Vec<Player>, // The players playing
}