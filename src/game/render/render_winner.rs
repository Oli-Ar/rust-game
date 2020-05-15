use piston_window::{ PistonWindow, Text, Transformed, Glyphs, Size, Event, clear };
use crate::game::Game;

pub fn render_winner (window: &mut PistonWindow,
                 e: Event,
                 window_size: Size,
                 game: &mut Game,
                 text: Text,
                 glyphs: &mut Glyphs
) {
  window.draw_2d(&e, |c, g, device| {
    clear([1.0, 1.0, 1.0, 1.0], g); // Creates white background

    let winner_text: &str = &format!("Player {} wins!", game.top_player.as_ref().unwrap().number);
    // Defines the transformations to position the texts
    let text_transform = c.transform.trans(window_size.width/2.0-117.0, ((window_size.height+50.0)/2.0)-15.0);

    // Draws the text
    text.draw(winner_text, glyphs, &c.draw_state, text_transform, g).unwrap();
    glyphs.factory.encoder.flush(device);
  });
}