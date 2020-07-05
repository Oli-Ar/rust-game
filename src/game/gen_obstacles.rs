use rand_distr::{ Normal, Distribution };
use rand::{ thread_rng, Rng };

pub fn gen_obstacles(x: &i32) -> Vec<Vec<i32>> {
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
      if (cell + v > 0) && (cell + v < x*x-1) && (v != 0) && (cell != x*x-1)
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