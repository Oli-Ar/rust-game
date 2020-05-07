//TODO: Clean up imports to only import exactly what needed
extern crate piston_window;
extern crate find_folder;
mod game_board;
mod player_amnt_input;

use game_board::render_board;
use player_amnt_input::get_input;
use piston_window::*;
use crate::structs::{Cell, Game};

pub fn render(mut game: Game, cells: Vec<Cell>) {
  // Opens a 512x512 pixel window called game
  let mut window: PistonWindow =
      WindowSettings::new("Game", [512; 2])
          .build().unwrap();

  //TODO: Find a better way to store data to draw shapes and text

  // Fetches the font for the number of squares to be written in
  let assets = find_folder::Search::ParentsThenKids(3, 3)
      .for_folder("assets").unwrap();
  let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

  // Defines the border of the rectangle as black with 100% opacity and width of 1
  let border = rectangle::Border{
    color: [0.0, 0.0, 0.0, 1.0],
    radius: 1.0,
  };

  // Defines the rectangle as being opaque white, with 90 degree corners and the pre defined border
  let rect = Rectangle{
    color: [1.0, 1.0, 1.0, 1.0],
    shape: rectangle::Shape::Square,
    border: Some(border),
  };

  // Defines the parameters of the text to be rendered
  let text_to_render = Text{
    color: [0.0, 0.0, 0.0, 1.0],
    font_size: 40,
    round: true,
  };
  let small_text = Text {
    font_size: 20,
    ..text_to_render
  };

  // While events can take place perform actions to window
  while let Some(e) = window.next() {
    // Gets the size of the window to be used in resizing the game board
    let window_size = window.size();
    if game.active == false {
      get_input(&mut window, e, window_size, &mut game, text_to_render, small_text, &mut glyphs);
    } else {
      render_board(&mut window, e, &cells, window_size, &mut glyphs, rect, text_to_render);
    }
  };
}