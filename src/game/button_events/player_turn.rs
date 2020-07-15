use crate::game::Game;
use crate::structs::{ Player, RollData, ObstacleData };
use rand::{ Rng, thread_rng };
use crate::structs::Obstacle::{Snake, Ladder};

impl Game {
  pub fn player_turn(&mut self, player_num: i32) {
    // The option has ownership of the player vector so to edit it a new option has to replace it
    // old option is cloned so it can be edited, clone will replace old option
    let mut player_vec: Vec<Player> = self.players.clone().unwrap();
    let mut player: &mut Player = &mut player_vec[(player_num-1) as usize];
    // Takes the old score before it is updated with the new roll data
    let old_score: i32 = player.score;
    // Calls on function to roll dice
    let roll: Vec<i32> = roll_dice(self.game_options.dice_sides);
    // Updates the players position and returns the new score
    let new_score = update_pos(player, roll[2], &self);
    // Checks if the player landed on an obstacle and returns a vector of those obstacle_images
    let obstacle_data = check_obstacle(player, &self, Vec::new());
    let obstacles = if obstacle_data.len() > 0 { Some(obstacle_data) } else { None };
    // Updates the top player so game ends if player finished
    if self.top_player.is_none() || player.score > self.top_player.as_ref().unwrap().score {
      self.top_player = Some(player.clone());
    }
    // Updates players roll data so it can be rendered
    player.roll = Some(RollData{
      roll_one: roll[0], roll_two: roll[1], roll_total: roll[2], old_score, new_score, obstacles,
    });
    // Replaces the old players option vector
    self.players = Some(player_vec);
  }
}

fn update_pos(player: &mut Player, roll_total: i32, game: &Game) -> i32 {
  // Updates the players score ensuring it stays above or at 0
  player.score = if player.score + roll_total > 0 { player.score + roll_total } else { 0 };
  let new_score: i32;
  // Updates the players cell, if score is 0 the cell is set to none
  player.cell = if player.score > 0 && player.score <= game.game_options.board_size*game.game_options.board_size {
    new_score = player.score;
    Some(game.cells[(player.score-1) as usize].clone())
  } else {
    new_score = 0;
    None
  };
  return new_score;
}

fn check_obstacle(player: &mut Player, game: &Game, mut ob_vec: Vec::<ObstacleData>) -> Vec<ObstacleData> {
  // Checks that player is on a cell
  if player.cell.as_ref().is_some() {
    let target_cell = player.cell.as_ref().unwrap().clone();
    // If the cell the player is on has an obstacle the type is recorded as ladder if the obstacle
    // moves the player forwards and as snake if it moves player back
    if target_cell.start == true {
      // Calls the update position function to update the players position to the end of the obstacle
      update_pos(player, target_cell.end.unwrap()+1 - target_cell.cell_number, &game);
      let obstacle = if target_cell.end.unwrap()+1 - target_cell.cell_number > 0 {
        Ladder
      } else {
        Snake
      };

      // Pushes the obstacle data to the vector to be returned
      ob_vec.push(ObstacleData{
        obstacle,
        start: target_cell.cell_number,
        end: target_cell.end.unwrap()+1
      });
      // Recursively runs the check_obstacle function to check if the obstacle leads to another
      ob_vec = check_obstacle(player, &game, ob_vec);
    };
  }
  // Returns the vector of obstacle data
  return ob_vec.clone();
}

fn roll_dice(dice_side: i32) -> Vec<i32> {
  // Generates two values between and including 1 and 6 to simulate dice, returned as vec to be
  // stored as roll data
  let mut return_vec: Vec<i32> = Vec::with_capacity(3);
  let rng = thread_rng();
  let roll_one: i32 = rng.clone().gen_range(1, dice_side+1);
  let roll_two: i32 = rng.clone().gen_range(1, dice_side+1);
  return_vec.extend(vec![roll_one, roll_two]);
  let roll_total: i32;
  if roll_one == roll_two {
    roll_total = -1 * (roll_one + roll_two);
  } else {
    roll_total = roll_one + roll_two;
  };
  return_vec.push(roll_total);
  return return_vec;
}