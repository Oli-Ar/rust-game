use std::fmt;

#[derive(Debug, Clone)]
// Data to be stored in each cell to correctly render the game
pub struct Cell {
  pub cell_number: i32, // Number of the cell on 7*7 board (1-49)
  pub x: i32, // x position on a 7*7 board
  pub y: i32, // y position on a 7*7 board
  pub start: bool, // Whether the obstacle is a start of an obstacle
  pub end: Option<i32> // Number of the cell the obstacle ends on
}

#[derive(Debug, Clone)]
// Data to be stored about each player
pub struct Player {
  pub number: i32, // The players number between 1 and 4
  pub score: i32, // The score and therefore cell of the player
  pub cell: Option<Cell>, // THe cell the player is on, none at start
  pub roll: Option<RollData>
}

#[derive(Debug, Clone)]
// Data used for player turns
pub struct RollData {
  pub roll_one: i32, // Value of the first roll
  pub roll_two: i32, // Value of the second roll
  pub roll_total: i32, // Value of both rolls - negative if roll one and two are equal
  pub old_score: i32, // The cell the player was on
  pub new_score: i32, // The cell the player moved to
  pub obstacles: Option<Vec<ObstacleData>> // Any obstacles on the cell the player landed on
}

#[derive(Debug, Clone)]
// Data about obstacles the player used
pub struct ObstacleData {
  pub obstacle: Obstacle, // The type of obstacle
  pub start: i32, // The start cell of the obstacle
  pub end: i32, // The end cell of the obstacle
}

#[derive(Debug, Clone)]
// Two possible types of obstacles
pub enum Obstacle {
  Snake,
  Ladder,
}

// Implementation allowing for obstacle be converted to a string
impl fmt::Display for Obstacle {
  // Uses the debug print to convert an enum to a string
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}