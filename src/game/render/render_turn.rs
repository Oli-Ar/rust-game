use piston_window::{ PistonWindow, Event, Text, Size, Glyphs, Transformed, ButtonState, ButtonEvent };
use crate::game::Game;

pub fn render_turn(game: &mut Game,
                   window: &mut PistonWindow,
                   window_size: Size,
                   glyphs: &mut Glyphs,
                   e: &Event,
                   text: Text,
                   turn: i32
) -> i32 {
  window.draw_2d(e, |c, g, _device| {
    // Converts player number to string and formats into a str - defines position of text and draws it
    let text_to_draw: &str = &format!("Player {}'s turn, press space to roll the dice.", turn.to_string());
    let transform = c.transform.trans(window_size.width/2.0-160.0, 33.0);
    text.draw(text_to_draw, glyphs, &c.draw_state, transform, g).unwrap();
  });

  let mut new_turn = turn;

  // Check for keypress event
  if let Some(k) = e.button_args() {
    if k.state == ButtonState::Press {
      let res = game.turn_keypress(&k.button, turn);
      if res != None { new_turn = res.unwrap() }
    }
  };
  return new_turn;
}