extern crate piston_window;

use piston_window::*;

fn main() {
  let mut cells: Vec<Cell> = Vec::with_capacity(49); // Vector containing 49 cells that make up the game board
  for i in 0..49 {
    // A number between 1 and 49 that matches the order of the game (https://prnt.sc/sbeizh)
    let cell_num: i16;
    if i / 7 % 2 == 1 { cell_num = 7 * (7 - i / 7) - (i % 7) } else { cell_num = 7 * (6 - i / 7) + (i % 7) + 1 };
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
  // While events can take place perform actions to window
  while let Some(e) = window.next() {
    window.draw_2d(&e, |c, g, _| {
      // Draw each cell in cells as a grid
      for i in &cells {
        let rect = [(i.x * (512 / 7)) as f64, (i.y * (512 / 7)) as f64, (512 / 7) as f64, (512 / 7) as f64];
        Rectangle::new_border([1.0, 1.0, 1.0, 1.0], 1.0).draw(rect, &c.draw_state, c.transform, g);
      }
    });
  }
}

#[allow(dead_code)]
struct Cell {
  cell_number: i16, // Number of the cell on 7*7 board (1-49)
  x: i16, // x position on a 7*7 board
  y: i16, // y position on a 7*7 board
}
