extern crate piston_window;

use piston_window::{ PistonWindow, Rectangle, Text, Transformed, Glyphs, Size, Event, Context, G2d };
use crate::structs::Cell;
use crate::game::Game;

pub fn render_board(window: &mut PistonWindow,
                    e: &Event,
                    game: &Game,
                    window_size: Size,
                    glyphs: &mut Glyphs,
                    rect: Rectangle,
                    no_tex: Text,
) {
  window.draw_2d(e, |c, g, device| {
    // Draw each cell in cells as a grid
    for i in &game.cells {
      // Separate functions to render different sections of board
      draw_cells(game, rect, window_size, i, c, g);
      draw_nums(game, no_tex, window_size, i, glyphs, c, g);
    };
    glyphs.factory.encoder.flush(device);
  });
}

fn draw_cells(game: &Game, rect: Rectangle, window_size: Size, i: &Cell, c: Context, g: &mut G2d) {
  // Defines the size of the rectangle as 1/7th the size of the window to allow 7*7 grid
  let rect_size = [
    i.x as f64*(window_size.width / game.board_size as f64), // Location on x axis of current cell
    i.y as f64*(window_size.height / game.board_size as f64), // Location on y axis of cell
    (window_size.width / game.board_size as f64), // Size of cell on x axis
    (window_size.height / game.board_size as f64)]; // Size of cell on y axis

  rect.draw(rect_size, &c.draw_state, c.transform, g);
}

fn draw_nums(game: &Game, text: Text, window_size: Size, i: &Cell, glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
  // Converts the int of the current square to a string which can be coerced into str
  let sqr_num = i.cell_number.to_string();
  // Defines the transformation for the position of the number
  let transform = c.transform.trans((i.x as f64 + 0.3)*window_size.width/game.board_size as f64,
                                    window_size.height/game.board_size as f64/1.5+(i.y as f64+0.1)*window_size.height/game.board_size as f64);

  // Renders and displays the number of the square
  text.draw(&sqr_num, glyphs, &c.draw_state, transform, g).unwrap();
}