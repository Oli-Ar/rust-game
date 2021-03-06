mod game_start;
mod player_turn;

use crate::game::Game;
use piston_window::{Button, Key};

impl Game {
    // Method to check how many players in game based on user input
    pub fn init_keypress(&mut self, btn: &Button) {
        // Match statement to check if the key is a number key in the range 2-4, runs function to config game
        match btn {
            &Button::Keyboard(Key::D2) => self.config_game(2),
            &Button::Keyboard(Key::D3) => self.config_game(3),
            &Button::Keyboard(Key::D4) => self.config_game(4),
            _ => {}
        };
    }

    pub fn turn_keypress(&mut self, btn: &Button, player_num: i32) -> Result<i32, ()> {
        if btn == &Button::Keyboard(Key::Space) {
            self.player_turn(player_num);
            return if player_num == self.player_count.unwrap() {
                Ok(1)
            } else {
                Ok(player_num + 1)
            };
        } else {
            Err(())
        }
    }
}
