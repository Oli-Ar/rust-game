use crate::game::Game;
use crate::structs::{ Player, RollData };
use rand::Rng;

impl Game {
  pub fn player_turn(&mut self, player_num: i32) {
    let mut player_vec: Vec<Player> = self.players.clone().unwrap();
    let player: &mut Player = &mut player_vec[(player_num-1) as usize];
    let old_score: i32 = player.score;
    let roll: Vec<i32> = roll_dice();
    player.score += roll[2];
    let new_score: i32;
    if player.score > 0 && player.score <= self.board_size*self.board_size {
      new_score = player.score;
      player.cell = Some(self.cells[(player.score-1) as usize].clone());
    } else {
      new_score = 0;
      player.cell = None
    };
    if self.top_player.is_none() || player.score > self.top_player.as_ref().unwrap().score {
      self.top_player = Some(player.clone());
    }
    player.roll = Some(RollData{
      roll_one: roll[0], roll_two: roll[1], roll_total: roll[2], old_score, new_score
    });
    self.players = Some(player_vec);
  }
}

fn roll_dice() -> Vec<i32> {
  let mut return_vec: Vec<i32> = Vec::with_capacity(3);
  let rng = rand::thread_rng();
  let roll_one: i32 = rng.clone().gen_range(1, 7);
  let roll_two: i32 = rng.clone().gen_range(1, 7);
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