mod render_board;
mod structs;
mod new_cells_vec;
mod quick_sort;

use std::thread;
use structs::Cell;
use render_board::render_board;
use new_cells_vec::make_cells_vec;
use quick_sort::quick_sort;
use std::thread::JoinHandle;

fn main() {
  // Uses the make cells vec function to make a vec of cells then sorts the cells into order using a quick sort
  let mut cells: Vec<Cell> = make_cells_vec();
  let vec_len: i64 = (cells.len()-1) as i64;
  quick_sort(&mut cells, 0, vec_len);

  // Creates new vec to store opened threads
  let mut open_threads: Vec<JoinHandle<()>> = Vec::new();
  // Uses the render board module to render the game board
  open_threads.push(thread::spawn(|| {
    render_board(cells);
  }));
  
  #[allow(unused_must_use)]
  // Closes open threads once they have ran
  for thread in open_threads {
    thread.join();
  };
}