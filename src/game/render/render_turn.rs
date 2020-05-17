use piston_window::{ PistonWindow, Event, Text, Size, Glyphs, Transformed, ButtonState, ButtonEvent };
use crate::game::Game;

pub fn render_turn(game: & mut Game,
                   window: &mut PistonWindow,
                   window_size: Size,
                   glyphs: &mut Glyphs,
                   e: &Event,
                   mut text: Text,
                   turn: i32
) -> i32 {
  let mut new_turn = turn;

  // Check for keypress event - returns an i32 representing which players turn it is
  if let Some(k) = e.button_args() {
    if k.state == ButtonState::Press {
      let res = game.turn_keypress(&k.button, turn);
      if res != None { new_turn = res.unwrap() }
    }
  };

  window.draw_2d(e, |c, g, _device| {
    // Converts player number to string and formats into a str - defines position of text and draws it
    text.font_size = 14;
    let top_text: &str = &format!("Player {}'s turn, press space to roll the dice.", turn.to_string());
    let top_transform = c.transform.trans(window_size.width/2.0-227.0, 33.0);
    text.draw(top_text, glyphs, &c.draw_state, top_transform, g).unwrap();

    // Checks the previous players roll if it is some and displays it at the bottom of the window
    let prev_player = if turn == 1 { game.player_count.unwrap()-1 } else { turn-2 };
    let player = &game.players.as_ref().unwrap()[(prev_player) as usize];
    if player.roll.is_some() {
      let roll_data = player.roll.as_ref().unwrap();
      text.font_size = 10;
      let bottom_text: &str = &format!(
        "Player {} rolled ({}, {}), total: {}, and moved from square {} to square {}",
        player.number,
        roll_data.roll_one, roll_data.roll_two,
        roll_data.roll_total,
        roll_data.old_score, roll_data.new_score
      );
      let bottom_transform = c.transform.trans(window_size.width/2.0-234.0, window_size.height+13.0);
      text.draw(bottom_text, glyphs, &c.draw_state, bottom_transform, g).unwrap();

      // Checks if on the previous roll the player landed on an obstacle
      if roll_data.obstacles.is_some() {
        let obstacles = roll_data.obstacles.clone().unwrap();
        // For each of the obstacles the player landed on the type, position of the obstacle, and
        // where it took the player is rendered to the bottom of the screen
        for (i, o) in obstacles.iter().enumerate() {
          let obstacle_text = &format!(
            "Square {} was the start of a {} moving player {} to square {}",
            o.start, o.obstacle.to_string().to_ascii_lowercase(), player.number, o.end
          );
          let obstacle_transform = c.transform.trans(window_size.width/2.0-215.0, window_size.height+((2.0+i as f64)*13.0));
          text.draw(obstacle_text, glyphs, &c.draw_state, obstacle_transform, g).unwrap();
        }
      }
    }
  });
  return new_turn;
}