use crate::structs::Cell;

// Static function owned by Game struct used to create a vector of cells
pub fn make_cells_vec(x: i32) -> Vec<Cell> {
  let mut cells_vec: Vec<Cell> = Vec::with_capacity((x*x) as usize); // Makes vec with a capacity of 19
  for i in 0..(x*x) {
    // A number between 1 and 49 that matches the order of the game (https://prnt.sc/sbeizh)
    let cell_num: i32;
    if x%2 == 1 {
      if i/x%2 == 1 { cell_num = x*(x-i/x)-(i%x) } else { cell_num = x*((x-1)-i/x)+(i%x)+1 };
    } else {
      if i/x%2 == 1 { cell_num = x*((x-1)-i/x)+(i%x)+1 } else { cell_num = x*(x-i/x)-(i%x) };
    };
    let new_cell: Cell = Cell {
      cell_number: cell_num,  
      x: i % x, // Value between 0 and 6 which will be used to plot the cell on the x axis
      y: i / x, // Value between 0 and 6 which will be used to plot the cell on the y axis
    };
    cells_vec.push(new_cell); // Push cell to the vector of cells
  }
  return cells_vec;
}