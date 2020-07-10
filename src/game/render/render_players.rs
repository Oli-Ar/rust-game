use piston_window::{ PistonWindow, Ellipse, ellipse, Size, Event, Context, G2d };
use crate::game::Game;

// Defines the border of the counter as 1 pixel thick black with 100% opacity
const BORDER: ellipse::Border = ellipse::Border {
  color: [0.0, 0.0, 0.0, 1.0],
  radius: 1.0,
};

// Defines the base values of the cylinder
const COUNTER: Ellipse = Ellipse {
  color: [0.0, 0.0, 0.0, 0.0],
  border: Some(BORDER),
  resolution: 160
};

pub fn render_players(game: &mut Game, window: &mut PistonWindow, window_size: Size, e: &Event) {
  window.draw_2d(e, |c, mut g, _device| {
    // Loops through all players and checks if and what cell they are on
    for player in game.players.as_ref().unwrap() {
      if let Some(cell) = &player.cell {

        // Using data from the cell the player is on the size and position of counter is defined
        let counter_size = (window_size.width+window_size.height)/2.0/(game.board_size*6) as f64;
        let cell_x = cell.x as f64*(window_size.width / game.board_size as f64);
        let cell_y = cell.y as f64*((window_size.height-50.0) / game.board_size as f64)+50.0;

        // Match statement to check player number and then draw the counter in a certain colour
        // Player 1 is drawn top left, 2: top right, 3: bottom left, 4: bottom right
        match player.number {
          1 => {
            draw_circle([1.0, 0.0, 1.0, 1.0],
                        counter_size,
                        cell_x+5.0,
                        cell_y+5.0,
                        c, &mut g
            );
          },
          2 => {
            draw_circle([0.0, 1.0, 1.0, 1.0],
                        counter_size,
                        cell_x+window_size.width/game.board_size as f64-counter_size-5.0,
                        cell_y+5.0,
                        c, &mut g
            );
          },
          3 => {
            draw_circle([0.0, 0.0, 1.0, 1.0],
                        counter_size,
                        cell_x+5.0,
                        cell_y+(window_size.height-50.0)/game.board_size as f64-counter_size-5.0,
                        c, &mut g
            );
          },
          4 => {
            draw_circle([1.0, 1.0, 0.0, 1.0],
                        counter_size,
                        cell_x+window_size.width/game.board_size as f64-counter_size-5.0,
                        cell_y+(window_size.height-50.0)/game.board_size as f64-counter_size-5.0,
                        c, &mut g
            );
          },
          _ => { /* Not possible, does nothing */ }
        }
      }
    }
  });
}

// Function to draw a circle representing a player
fn draw_circle(color: [f32; 4], size: f64, x_pos: f64, y_pos: f64, c: Context, g: &mut G2d<'_>) {
  let circle = Ellipse { color, ..COUNTER };

  let counter_pos: [f64; 4] = [
    x_pos, // Location on x axis of current cell
    y_pos, // Location on y axis of cell
    size, // Size of cell on x axis
    size // Size of cell on y axis
  ];

  circle.draw(counter_pos, &c.draw_state, c.transform, g);
}