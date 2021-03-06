use crate::game::Game;
use piston_window::{
    clear, ButtonEvent, ButtonState, Event, Glyphs, PistonWindow, Size, Text, Transformed,
};

pub fn get_input(
    game: &mut Game,
    window: &mut PistonWindow,
    window_size: Size,
    e: Event,
    glyphs: &mut Glyphs,
    mut text: Text,
) {
    window.draw_2d(&e, |c, g, device| {
        clear([0.47, 0.87, 0.47, 1.0], g); // Creates white background

        // Defines the transformations to position the texts
        let top_text = c.transform.trans(
            window_size.width / 2.0 - 145.0,
            ((window_size.height + 50.0) / 2.0) - 30.0,
        );
        let bottom_text = c.transform.trans(
            window_size.width / 2.0 - 169.0,
            ((window_size.height + 50.0) / 2.0) + 10.0,
        );

        // Draws the text - fetches the messages from the hashmap of messages stored in game struct
        text.draw(
            &*game.messages.get("render_player_prompt_top").unwrap(),
            glyphs,
            &c.draw_state,
            top_text,
            g,
        )
        .unwrap();
        text.font_size = 14;
        text.draw(
            &*game.messages.get("render_player_prompt_bottom").unwrap(),
            glyphs,
            &c.draw_state,
            bottom_text,
            g,
        )
        .unwrap();
        glyphs.factory.encoder.flush(device);
    });

    // Check for keypress event
    if let Some(k) = e.button_args() {
        if k.state == ButtonState::Press {
            game.init_keypress(&k.button); // Run game method on button press
        }
    }
}
