use piston_window::{PistonWindow, WindowSettings, Window, Rectangle, rectangle, Text };
use crate::game::Game;

mod game_board;
mod player_amnt_input;
mod render_players;
mod render_turn;
mod render_winner;
mod render_obstacles;

use game_board::render_board;
use player_amnt_input::get_input;
use render_players::render_players;
use render_turn::render_turn;
use render_winner::render_winner;
use render_obstacles::render_obstacles;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];


impl Game {
  pub fn render(&mut self) {
    // Opens a 512x512 pixel window called game
    let mut window: PistonWindow = WindowSettings::new("Game", [512, 612]).build().unwrap();

    // Fetches the font for the number of squares to be written in
    let font_assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut glyphs = window.load_font(font_assets.join("Mono.ttf")).unwrap();

    // Defines the base shapes and text to be rendered
    let border = rectangle::Border { color: BLACK, radius: 1.0 };
    let rect = Rectangle { color: WHITE, shape: rectangle::Shape::Square, border: Some(border) };
    let text_to_render = Text { color: BLACK, font_size: 25, round: true };

    /*
    Parameter order for render modules:
    1. Game (self)
    2. window
    3. window size
    4. event
    5. assets
      1. shapes
      2. glyphs
      3. text
      4. images
    6. other
    */

    // While events can take place perform actions to window
    let mut turn: i32 = 1;
    while let Some(e) = window.next() {
      // Gets the size of the window to be used in resizing the game board
      let mut window_size = window.size();
      window_size.height -= 50.0;

      // Checks if game is running
      if self.active == false {
        get_input(self, &mut window, window_size, e, &mut glyphs, text_to_render);
      } else if self.active == true && self.top_player.as_ref().unwrap().score < self.board_size*self.board_size {
        render_board(self, &mut window, window_size, &e, rect, &mut glyphs,text_to_render);
        render_players(self, &mut window, window_size, &e);
        render_obstacles(&self, &mut window, window_size, &e);
        turn = render_turn(self, &mut window, window_size, &e, &mut glyphs, text_to_render, turn);
      } else {
        render_winner(self, &mut window, window_size, e,  &mut glyphs, text_to_render);
      }
    };
  }
}