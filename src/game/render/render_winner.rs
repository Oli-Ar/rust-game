use crate::game::Game;
use piston_window::{clear, Event, Glyphs, PistonWindow, Size, Text, Transformed};

pub fn render_winner(
    game: &mut Game,
    window: &mut PistonWindow,
    window_size: Size,
    e: Event,
    glyphs: &mut Glyphs,
    text: Text,
) {
    window.draw_2d(&e, |c, g, device| {
        // Creates blue background
        // Fetches the winner message from the hashmap stored in the game struct and replaces the empty
        // field with the player who won
        clear([0.47, 0.75, 0.87, 1.0], g);
        let winner_text: &str = &*game.messages.get("render_winner").unwrap().replace(
            "{player}",
            &*game.top_player.as_ref().unwrap().number.to_string(),
        );
        // Defines the transformations to position the texts
        let text_transform = c.transform.trans(
            window_size.width / 2.0 - 117.0,
            ((window_size.height + 50.0) / 2.0) - 15.0,
        );

        // Draws the text
        text.draw(winner_text, glyphs, &c.draw_state, text_transform, g)
            .unwrap();
        glyphs.factory.encoder.flush(device);
    });
}
