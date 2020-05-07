//TODO: Clean up imports to only import exactly what needed
extern crate piston_window;
use piston_window::*;
use crate::structs::Game;

pub fn get_input(window: &mut PistonWindow,
                 e: Event,
                 window_size: Size,
                 game: &mut Game,
                 large_text: Text,
                 small_text: Text,
                 glyphs: &mut Glyphs
) {
  window.draw_2d(&e, |c, g, device| {
    clear([1.0, 1.0, 1.0, 1.0], g); // Creates white background

    // Defines the transformations to position the texts
    let top_text = c.transform.trans(window_size.width/2.0-165.0, (window_size.height/2.0)-30.0);
    let bottom_text = c.transform.trans(window_size.width/2.0-160.0, (window_size.height/2.0)+10.0);

    //TODO: Store text externally

    // Draws the text
    large_text.draw("How many players?", glyphs, &c.draw_state, top_text, g).unwrap();
    small_text.draw("Press a number key between 2 and 4", glyphs, &c.draw_state, bottom_text, g).unwrap();
    glyphs.factory.encoder.flush(device);
  });

  // Check for keypress event
  if let Some(k) = e.button_args() {
    if k.state == ButtonState::Press {
      game.pressed(&k.button); // Run game method on button press
    }
  }
}