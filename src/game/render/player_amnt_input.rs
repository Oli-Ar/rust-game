use piston_window::{ PistonWindow, ButtonState, ButtonEvent, Text, Transformed, Glyphs, Size, Event, clear };
use crate::game::Game;

pub fn get_input(window: &mut PistonWindow,
                 e: Event,
                 window_size: Size,
                 game: &mut Game,
                 mut text: Text,
                 glyphs: &mut Glyphs
) {
  window.draw_2d(&e, |c, g, device| {
    clear([1.0, 1.0, 1.0, 1.0], g); // Creates white background

    // Defines the transformations to position the texts
    let top_text = c.transform.trans(window_size.width/2.0-145.0, ((window_size.height+50.0)/2.0)-30.0);
    let bottom_text = c.transform.trans(window_size.width/2.0-169.0, ((window_size.height+50.0)/2.0)+10.0);

    // Draws the text
    text.draw("How many players?", glyphs, &c.draw_state, top_text, g).unwrap();
    text.font_size = 14;
    text.draw("Press a number key between 2 and 4", glyphs, &c.draw_state, bottom_text, g).unwrap();
    glyphs.factory.encoder.flush(device);
  });

  // Check for keypress event
  if let Some(k) = e.button_args() {
    if k.state == ButtonState::Press {
      game.init_keypress(&k.button); // Run game method on button press
    }
  }
}