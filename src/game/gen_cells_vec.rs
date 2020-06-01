use crate::structs::Cell;
use rand_distr::{ Normal, Distribution };
use rand::{ thread_rng, Rng };

// Static function owned by Game struct used to create a vector of cells
// Here x is the length and width of the game board
pub fn make_cells_vec(x: i32) -> Vec<Cell> {
  // Makes vec with a capacity of x^2
  let mut cells_vec: Vec<Cell> = Vec::with_capacity((x*x) as usize);
  for i in 0..(x*x) {
    // Finds the x value of the cell so that it matches the order of the game so the counters
    // can move in a logical way
    let x_val: i32;
    if ( ((x-1)-i/x)%2 == 1 && x%2 == 1 ) || ( ((x-1)-i/x)%2 == 0 && x%2 == 0 ) {
      x_val = (x-1)-i%x
    } else {
      x_val = i%x
    };

    let new_cell: Cell = Cell {
      cell_number: i+1, // The number of the cell
      x: x_val, // Value between 0 and x which will be used to plot the cell on the x axis
      y: x-1-i/x, // Value between 0 and x which will be used to plot the cell on the y axis
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

fn gen_obstacles(x: i32) -> Vec<Vec<i32>> {
  let mut rng = thread_rng();
  // Generates a normal for generating arrows with a standard deviation of 15
  let normal = Normal::new(0.0, 15.0).unwrap();
  let mut obs_vec: Vec<Vec<i32>> = Vec::with_capacity((x*x) as usize);
  for _i in 0..(x*2) {
    // Do-while loop that generates an arrow for a random cell
    // Exit condition is: the arrow being longer than 0, not pointing to the same cell as another arrow,
    // arrow not pointing off the board, and not pointing to the end of a previous arrow, however
    // another arrow can start at the end of another arrow - the check is in place to prevent arrows
    // pointing at each other and leaving the player in an infinite loop leading to stack overflow
    loop {
      let cell: i32 = rng.gen_range(9, x*x);
      let v: i32 = ((normal.sample(&mut thread_rng()) as f64).round()) as i32;
      if (cell + v > 0) && (cell + v < x*x-1) && (v != 0) && (cell != x*x)
      && (obs_vec.iter().all(|i| {
        i[0]+i[1] != cell+v && i[0] != cell+v
      })) {
        obs_vec.push(vec![cell, v]);
        break;
      };
    };
  };
  return obs_vec;
}