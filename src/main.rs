extern crate piston_window;
extern crate find_folder;

use piston_window::*;

fn main() {
  let mut cells: Vec<Cell> = Vec::with_capacity(49); // Vector containing 49 cells that make up the game board
  for i in 0..49 {
    // A number between 1 and 49 that matches the order of the game (https://prnt.sc/sbeizh)
    let cell_num: i16;
    if i/7%2 == 1 { cell_num = 7*(7-i/7)-(i%7) } else { cell_num = 7*(6-i/7)+(i%7)+1 };
    let new_cell: Cell = Cell {
      cell_number: cell_num,
      x: i % 7, // Value between 0 and 6 which will be used to plot the cell on the x axis
      y: i / 7, // Value between 0 and 6 which will be used to plot the cell on the y axis
    };
    cells.push(new_cell); // Push cell to the vector of cells
  }

  // Opens a 512x512 pixel window called game
  let mut window: PistonWindow =
      WindowSettings::new("Game", [512; 2])
          .build().unwrap();

  // Fetches the font for the number of squares to be written in
  let assets = find_folder::Search::ParentsThenKids(3, 3)
      .for_folder("assets").unwrap();
  let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

  // While events can take place perform actions to window
  while let Some(e) = window.next() {
    window.draw_2d(&e, |c, g, device| {
      // Gets the size of the window to be used in resizing the game board
      let window_size = c.get_view_size();

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

      // Draw each cell in cells as a grid
      for i in &cells {
        // Defines the size of the rectangle as 1/7th the size of the window to allow 7*7 grid
        let rect_size = [
          i.x as f64* (window_size[0] / 7.0), // Location on x axis of current cell
          i.y as f64*(window_size[1] / 7.0), // Location on y axis of cell
          (window_size[0] / 7.0), // Size of cell on x axis
          (window_size[1] / 7.0)]; // Size of cell on y axis
        rect.draw(rect_size, &c.draw_state, c.transform, g);

        // Converts the int of the current square to a string which can be coerced into str
        let sqr_num = i.cell_number.to_string();
        // Defines the transformation for the position of the number
        let transform = c.transform.trans((i.x as f64 + 0.3)*window_size[0]/7.0,
                                          window_size[1]/7.0/1.5+(i.y as f64+0.1)*window_size[1]/7.0);

        // Renders and displays the number of the square
        text_to_render.draw(&sqr_num, &mut glyphs, &c.draw_state, transform, g).unwrap();
        glyphs.factory.encoder.flush(device);
      };
    });
  };
}

#[allow(dead_code)]
struct Cell {
  cell_number: i16, // Number of the cell on 7*7 board (1-49)
  x: i16, // x position on a 7*7 board
  y: i16, // y position on a 7*7 board
}
