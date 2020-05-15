use crate::game::Game;
use crate::structs::Player;
use rand::Rng;

impl Game {
  pub fn player_turn(&mut self, player_num: i32) {
    let mut player_vec: Vec<Player> = self.players.clone().unwrap();
    let player: &mut Player = &mut player_vec[(player_num-1) as usize];
    let roll = roll_dice(player);
    player.score += roll;
    if player.score > 0 && player.score <= self.board_size*self.board_size {
      player.cell = Some(self.cells[(player.score-1) as usize].clone());
    } else {
      player.cell = None
    };
    if self.top_player.is_none() || player.score > self.top_player.as_ref().unwrap().score {
      self.top_player = Some(player.clone());
    }
    self.players = Some(player_vec);
  }
}

fn roll_dice(player: &Player) -> i32 {
  let rng = rand::thread_rng();
  let roll_one: i32 = rng.clone().gen_range(1, 7);
  let roll_two: i32 = rng.clone().gen_range(1, 7);

  let roll_total: i32;
  if roll_one == roll_two {
    roll_total = -1 * (roll_one + roll_two)
  } else {
    roll_total = roll_one + roll_two
  };
  return if (player.score + roll_total) < 0 { 0 } else { roll_total };
}