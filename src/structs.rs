#[derive(Debug, Clone)]
// Data to be stored in each cell to correctly render the game
pub struct Cell {
  pub cell_number: i32, // Number of the cell on 7*7 board (1-49)
  pub x: i32, // x position on a 7*7 board
  pub y: i32, // y position on a 7*7 board
}

#[derive(Debug, Clone)]
// Data to be stored about each player
pub struct Player {
  pub number: i32, // The players number between 1 and 4
  pub score: i32, // The score and therefore cell of the player
  pub cell: Option<Cell> // THe cell the player is on, none at start
}