//TODO: Clean up imports to only import exactly what needed
extern crate piston_window;

use piston_window::*;
use crate::structs::Cell;

pub fn render_board(window: &mut PistonWindow,
                    e: Event,
                    cells: &Vec<Cell>,
                    window_size: Size,
                    glyphs: &mut Glyphs,
                    rect: Rectangle,
                    text_to_render: Text,
) {
  window.draw_2d(&e, |c, g, device| {
    // Draw each cell in cells as a grid
    for i in &*cells {
      // Separate functions to render different sections of board
      draw_cells(rect, window_size, i, c, g);
      draw_nums(text_to_render, window_size, i, glyphs, c, g);
    };
    glyphs.factory.encoder.flush(device);
  });
}

fn draw_cells(rect: Rectangle, window_size: Size, i: &Cell, c: Context, g: &mut G2d) {
  // Defines the size of the rectangle as 1/7th the size of the window to allow 7*7 grid
  let rect_size = [
    i.x as f64*(window_size.width / 7.0), // Location on x axis of current cell
    i.y as f64*(window_size.height / 7.0), // Location on y axis of cell
    (window_size.width / 7.0), // Size of cell on x axis
    (window_size.height / 7.0)]; // Size of cell on y axis
  rect.draw(rect_size, &c.draw_state, c.transform, g);
}

fn draw_nums(text: Text, window_size: Size, i: &Cell, glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
  // Converts the int of the current square to a string which can be coerced into str
  let sqr_num = i.cell_number.to_string();
  // Defines the transformation for the position of the number
  let transform = c.transform.trans((i.x as f64 + 0.3)*window_size.width/7.0,
                                    window_size.height/7.0/1.5+(i.y as f64+0.1)*window_size.height/7.0);

  // Renders and displays the number of the square
  text.draw(&sqr_num, glyphs, &c.draw_state, transform, g).unwrap();
}