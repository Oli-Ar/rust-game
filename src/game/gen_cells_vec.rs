use crate::structs::Cell;
use crate::game::gen_obstacles::gen_obstacles;

// Static function owned by Game struct used to create a vector of cells
// Here x is the length and width of the game board
pub fn make_cells_vec(x: &i32) -> Vec<Cell> {
  // Makes vec with a capacity of x^2
  let mut cells_vec: Vec<Cell> = Vec::with_capacity((x*x) as usize);
  for i in 0..(x*x) {
    // Finds the x value of the cell so that it matches the order of the game so the counters
    // can move in a logical way
    let x_val: i32;
    if ( ((x-1)-i/(*x))%2 == 1 && x%2 == 1 ) || ( ((x-1)-i/(*x))%2 == 0 && x%2 == 0 ) {
      x_val = (x-1)-i%*x
    } else {
      x_val = i%*x
    };

    let new_cell: Cell = Cell {
      cell_number: i+1, // The number of the cell
      x: x_val, // Value between 0 and x which will be used to plot the cell on the x axis
      y: x-1-i/(*x), // Value between 0 and x which will be used to plot the cell on the y axis
      start: false,
      end: None,
    };
    cells_vec.push(new_cell); // Push cell to the vector of cells
  }
  // Sorts cells by their cell number
  // quick_sort(&mut cells_vec, 0, (x*x-1) as i64);
  // Generates a vector of obstacles and assigns them to the correct cells
  let obstacles = gen_obstacles(x);
  for i in obstacles {
    cells_vec[i[0] as usize].start = true;
    cells_vec[i[0] as usize].end = Some(i[0] + i[1]);
  };
  return cells_vec;
}