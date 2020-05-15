use piston_window::{ PistonWindow, WindowSettings, Window, Rectangle, rectangle, Text };
use crate::game::Game;

mod game_board;
mod player_amnt_input;
mod render_players;
mod render_turn;
mod render_winner;

use game_board::render_board;
use player_amnt_input::get_input;
use render_players::render_players;
use render_turn::render_turn;
use render_winner::render_winner;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];


impl Game {
  pub fn render(&mut self) {
    // Opens a 512x512 pixel window called game
    let mut window: PistonWindow = WindowSettings::new("Game", [512; 2]).build().unwrap();

    // Fetches the font for the number of squares to be written in
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

    // Defines the base shapes and text to be rendered
    let border = rectangle::Border { color: BLACK, radius: 1.0 };
    let rect = Rectangle { color: WHITE, shape: rectangle::Shape::Square, border: Some(border) };
    let text_to_render = Text { color: BLACK, font_size: 40, round: true };
    let small_text = Text { font_size: 20, ..text_to_render };
    let num_txt = Text { font_size: 220/self.board_size as u32, ..text_to_render };

    // While events can take place perform actions to window
    let mut turn: i32 = 1;
    while let Some(e) = window.next() {
      // Gets the size of the window to be used in resizing the game board
      let window_size = window.size();

      // Checks if game is running
      if self.active == false {
        get_input(&mut window, e, window_size, self, text_to_render, small_text, &mut glyphs);
      } else if self.active == true && self.top_player.as_ref().unwrap().score < self.board_size*self.board_size {
        render_board(&mut window, &e, self, window_size, &mut glyphs, rect, num_txt);
        render_players(self, &mut window, &e, window_size);
        turn = render_turn(self, &mut window, window_size, &mut glyphs, &e, small_text, turn);
      } else {
        render_winner(&mut window, e, window_size, self, text_to_render, &mut glyphs);
      }
    };
  }
}