use piston_window::{ PistonWindow, Rectangle, Text, Transformed, Glyphs, Size, Event, Context, G2d, clear, Line };
use crate::structs::Cell;
use crate::game::Game;

pub fn render_board(window: &mut PistonWindow,
                    e: &Event,
                    game: &Game,
                    window_size: Size,
                    glyphs: &mut Glyphs,
                    rect: Rectangle,
                    mut text: Text,
) {
  window.draw_2d(e, |c, g, device| {
    // Sets background to white
    clear([1.0, 1.0, 1.0, 1.0], g);
    text.font_size = 220/game.board_size as u32;

    // Draw each cell in cells as a grid
    for i in &game.cells {
      // Separate functions to render different sections of board
      draw_cells(game, rect, window_size, &i, c, g);
      draw_nums(game, text, window_size, &i, glyphs, c, g);
    };
    for i in &game.cells {
      if i.start == true { draw_line(&game, i, window_size, c, g)}
    }
    glyphs.factory.encoder.flush(device);
  });
}

fn draw_cells(game: &Game, rect: Rectangle, window_size: Size, i: &Cell, c: Context, g: &mut G2d<'_>) {
  // Defines the size of the rectangle as 1/7th the size of the window to allow 7*7 grid
  let rect_size = [
    i.x as f64*(window_size.width / game.board_size as f64), // Location on x axis of current cell
    i.y as f64*((window_size.height-50.0) / game.board_size as f64)+50.0, // Location on y axis of cell
    (window_size.width / game.board_size as f64), // Size of cell on x axis
    ((window_size.height-50.0) / game.board_size as f64)]; // Size of cell on y axis

  rect.draw(rect_size, &c.draw_state, c.transform, g);
}

fn draw_nums(game: &Game, text: Text, window_size: Size, i: &Cell, glyphs: &mut Glyphs, c: Context, g: &mut G2d<'_>) {
  // Converts the int of the current square to a string which can be coerced into str
  let sqr_num = i.cell_number.to_string();
  // Defines the transformation for the position of the number
  let transform = c.transform.trans(
    (i.x as f64 + 0.3)*window_size.width/game.board_size as f64,
    50.0+window_size.height/game.board_size as f64/1.5 +
        (i.y as f64+0.1)*(window_size.height-50.0)/game.board_size as f64
  );

  // Renders and displays the number of the square
  text.draw(&sqr_num, glyphs, &c.draw_state, transform, g).unwrap();
}

fn draw_line(game: &Game, cell: &Cell, window_size: Size, c: Context, g: &mut G2d<'_>) {
  // Sets a different colour and offset depending on if the line is a snake or a ladder
  let color: [f32; 4];
  let offset: f64;
  if cell.cell_number > game.cells[cell.end.unwrap() as usize].cell_number {
    color = [1.0, 0.0, 0.0, 1.0];
    offset = -10.0;
  } else {
    color = [0.0, 1.0, 0.0, 1.0];
    offset = 10.0;
  }
  let line = Line {
    color,
    radius: 2.0,
    shape: piston_window::line::Shape::Round
  };
  // Defines the coordinates of the arrow as [hx, hy, tx, ty] where h is head and t is tail
  // the arrows are offset to not overlap severely with each other
  let arrow = [
    (cell.x as f64 + 0.5)*window_size.width/game.board_size as f64 + offset,
    50.0+window_size.height/game.board_size as f64/2.0 +
      (cell.y as f64)*(window_size.height-50.0)/game.board_size as f64 + offset,
    (game.cells[cell.end.unwrap() as usize].x as f64 + 0.5)*window_size.width/game.board_size as f64 + offset,
    50.0+window_size.height/game.board_size as f64/2.0 +
        (game.cells[cell.end.unwrap() as usize].y as f64)*(window_size.height-50.0)/game.board_size as f64 + offset,
  ];
  line.draw_arrow(arrow, 5.0, &c.draw_state, c.transform, g)
}