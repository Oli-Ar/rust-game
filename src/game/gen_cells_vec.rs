use crate::structs::Cell;
use rand_distr::{ Normal, Distribution };
use rand::{ thread_rng, Rng };
use crate::game::quick_sort_cells::quick_sort;

// Static function owned by Game struct used to create a vector of cells
// Here x is the length and width of the game board
pub fn make_cells_vec(x: i32) -> Vec<Cell> {
  let mut cells_vec: Vec<Cell> = Vec::with_capacity((x*x) as usize); // Makes vec with a capacity of x^2
  for i in 0..(x*x) {
    // A number between 1 and x^2 that matches the order of the game
    let cell_num: i32;
    if x%2 == 1 {
      if i/x%2 == 1 { cell_num = x*(x-i/x)-(i%x) } else { cell_num = x*((x-1)-i/x)+(i%x)+1 };
    } else {
      if i/x%2 == 1 { cell_num = x*((x-1)-i/x)+(i%x)+1 } else { cell_num = x*(x-i/x)-(i%x) };
    };
    let new_cell: Cell = Cell {
      cell_number: cell_num,
      x: i % x, // Value between 0 and x which will be used to plot the cell on the x axis
      y: i / x, // Value between 0 and x which will be used to plot the cell on the y axis
      // start: false,
      // end: None,
    };
    cells_vec.push(new_cell); // Push cell to the vector of cells
  }
  quick_sort(&mut cells_vec, 0, (x*x-1) as i64);
  // let obstacles = gen_obstacles(x*x);
  // for i in obstacles {
  //   cells_vec[i[0] as usize].start = true;
  //   cells_vec[i[0] as usize].end = Some(&cells_vec[i[1] as usize]);
  // };
  println!("{:?}", cells_vec);
  return cells_vec;
}