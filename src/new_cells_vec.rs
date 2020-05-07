use crate::structs::Cell;

pub fn make_cells_vec() -> Vec<Cell> {
  let mut cells_vec: Vec<Cell> = Vec::with_capacity(49); // Makes vec with a capacity of 19
  for i in 0..49 {
    // A number between 1 and 49 that matches the order of the game (https://prnt.sc/sbeizh)
    let cell_num: i32;
    if i/7%2 == 1 { cell_num = 7*(7-i/7)-(i%7) } else { cell_num = 7*(6-i/7)+(i%7)+1 };
    let new_cell: Cell = Cell {
      cell_number: cell_num,
      x: i % 7, // Value between 0 and 6 which will be used to plot the cell on the x axis
      y: i / 7, // Value between 0 and 6 which will be used to plot the cell on the y axis
      player: None, // States no player is on the board currently
    };
    cells_vec.push(new_cell); // Push cell to the vector of cells
  }
  return cells_vec; // Returns a vector containing 49 cells
}