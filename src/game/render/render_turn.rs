use crate::game::Game;
use piston_window::{
    ButtonEvent, ButtonState, Event, Glyphs, PistonWindow, Size, Text, Transformed,
};

pub fn render_turn(
    game: &mut Game,
    window: &mut PistonWindow,
    window_size: Size,
    e: &Event,
    glyphs: &mut Glyphs,
    mut text: Text,
    turn: i32,
) -> i32 {
    window.draw_2d(e, |c, g, _device| {
        // Converts player number to string and formats into a str - defines position of text and draws it
        text.font_size = 14;
        // Fetches the message template from the messages hashmap stored in game struct and
        // replaces the fields to be replaced - this is done for all the messages
        let top_text: &str = &*game
            .messages
            .get("render_turn_top")
            .unwrap()
            .replace("{player}", &*turn.to_string());
        let top_transform = c.transform.trans(window_size.width / 2.0 - 227.0, 33.0);
        text.draw(top_text, glyphs, &c.draw_state, top_transform, g)
            .unwrap();

        // Checks the previous players roll if it is some and displays it at the bottom of the window
        let prev_player = if turn == 1 {
            game.player_count.unwrap() - 1
        } else {
            turn - 2
        };
        let player = &game.players.as_ref().unwrap()[(prev_player) as usize];
        if let Some(roll_data) = &player.roll {
            text.font_size = 10;
            let bottom_text: &str = &*game
                .messages
                .get("render_turn_bottom")
                .unwrap()
                .replace("{player}", &*player.number.to_string())
                .replace("{roll_one}", &*roll_data.roll_one.to_string())
                .replace("{roll_two}", &*roll_data.roll_two.to_string())
                .replace("{total}", &*roll_data.roll_total.to_string())
                .replace("{start}", &*roll_data.old_score.to_string())
                .replace("{end}", &*roll_data.new_score.to_string());
            let bottom_transform = c
                .transform
                .trans(window_size.width / 2.0 - 236.0, window_size.height + 13.0);
            text.draw(bottom_text, glyphs, &c.draw_state, bottom_transform, g)
                .unwrap();

            // Checks if on the previous roll the player landed on an obstacle
            if let Some(obstacles) = &roll_data.obstacles {
                // For each of the obstacle_images the player landed on the type, position of the obstacle, and
                // where it took the player is rendered to the bottom of the screen
                for (i, o) in obstacles.iter().enumerate() {
                    let obstacle_text: &str = &*game
                        .messages
                        .get("render_turn_obstacle")
                        .unwrap()
                        .replace("{start}", &*o.start.to_string())
                        .replace("{obstacle}", &*o.obstacle.to_string().to_ascii_lowercase())
                        .replace("{player}", &*player.number.to_string())
                        .replace("{end}", &*o.end.to_string());
                    let obstacle_transform = c.transform.trans(
                        window_size.width / 2.0 - 215.0,
                        window_size.height + ((2.0 + i as f64) * 13.0),
                    );
                    text.draw(obstacle_text, glyphs, &c.draw_state, obstacle_transform, g)
                        .unwrap();
                }
            }
        }
    });

    // Check for keypress event - returns an i32 representing which players turn it is
    if let Some(k) = e.button_args() {
        if k.state == ButtonState::Press {
            return match game.turn_keypress(&k.button, turn) {
                Ok(v) => v,
                Err(_) => turn,
            };
        }
    };
    turn
}
