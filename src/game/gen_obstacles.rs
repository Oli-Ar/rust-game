use crate::game::fetch_messages::read_lines;
use crate::game::GameOptions;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};
use regex::Regex;
use std::str::FromStr;

pub fn gen_obstacles(x: i32, game_options: &GameOptions) -> Vec<(i32, i32)> {
    let mut rng = thread_rng();
    // Generates a normal for generating arrows with a standard deviation of x*2
    let normal = Normal::new(0.0, (x * 2) as f64).unwrap();
    let mut obs_vec: Vec<(i32, i32)> = Vec::with_capacity((x * x) as usize);
    for _i in 0..=(x * 2) {
        if game_options.random_obstacles == false {
            break;
        };
        // Do-while loop that generates an arrow for a random cell. Exit condition is: the arrow being
        // longer than 0, not pointing to the same cell as another arrow, arrow not pointing off the
        // board, and not pointing to the end of a previous arrow, however another arrow can start at
        // the end of another arrow - the check is in place to prevent arrows pointing at each other
        // and leaving the player in an infinite loop leading to stack overflow
        loop {
            // Starting cell
            let cell: i32 = rng.gen_range(x, x * x);
            // The variation of the ladder from the cell
            let v: i32 = ((normal.sample(&mut thread_rng()) as f64).round()) as i32;
            if (cell + v > 0)
                && (cell + v < x * x - 1)
                && (v != 0)
                && (cell != x * x - 1)
                && (obs_vec
                    .iter()
                    .all(|i| i.0 + i.1 != cell + v && i.0 != cell + v))
            {
                obs_vec.push((cell, v));
                break;
            };
        }
    }
    // Obstacles defined by the user are parsed and put in a vec in the same format as the randomly
    // generated obstacles. The vec of random obstacles is then extended with user defined obstacles
    let user_obstacles: Vec<(i32, i32)> = fetch_obstacles(x * x);
    obs_vec.extend(user_obstacles);
    obs_vec
}

fn fetch_obstacles(x: i32) -> Vec<(i32, i32)> {
    let mut obstacle_vec: Vec<(i32, i32)> = Vec::new();
    if let Ok(lines) = read_lines("assets/obstacles.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(cell) = line {
                // Regex that greedily checks for 0 or more whitespaces followed by '//' and any text to
                // match comments
                let regex = Regex::new(r"\s*//.+").unwrap();
                // Checks next line if line is empty or a comment
                if cell.is_empty() || regex.is_match(&*cell) {
                    continue;
                };
                // Parses string from text file into an i32. The result type will fail if either value
                // failed to be parsed into an i32 when collected
                // https://doc.rust-lang.org/stable/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect
                type A = Result<Vec<i32>, <i32 as FromStr>::Err>;
                let parsed_cells: A = cell.split("->").map(|v| v.parse::<i32>()).collect();
                match parsed_cells {
                    Err(_) => {
                        println!(
                            "'{}' is not a valid obstacle. Define obstacle using start->end.",
                            cell
                        );
                        continue;
                    }
                    Ok(v) => {
                        // Checks that the value is on the board and if it is the start cell and the length of
                        // the obstacle are pushed to a vector which is then returned
                        if v[0] > x || v[1] > x {
                            println!(
                                "Board doesn't contain value where '{}' starts or ends.",
                                cell
                            );
                            continue;
                        }
                        obstacle_vec.push((v[0] - 1, v[1] - v[0]));
                    }
                };
            };
        }
    };
    obstacle_vec
}
