```
// Written orginally in a text file, ported to a markdown file to push to github
// Planned to build using rust piston_window crate so some structure constructors are based on the docs

structure Game {
    public field Cells : vector of Cell structures,
    public field game options : GameOption structure,
    public field active : boolean,
    public field player count : optional int,
    public field players : optional vector of Player structures,
    public field top player : optional Player structure,
    public field messages : Hashmap of messages (strings) with strings as keys

end structure
}

structure Cell {
   public field cell number : int,
   public field x : int,
   public field y : int,
   public field start : boolean,
   public field end : optional int,

end strcture
}

structure Player {
    public field number : int,
    public field score : int,
    public cell : optional Cell structure,
    public roll : optional RollData structure

end structure
}

structure RollData {
    public field roll_one : int,
    public field roll_two : int,
    public field roll_total : int,
    public field old_score : int,
    public field new_score : int,
    public field obstacles : optional vector of ObstacleData structures,

end structure
}

structure ObstacleData {
    public field obstacle : Obstacle,
    public field start : int,
    public field end : int

end structure
}

type Obstacle {
    Snake,
    Ladder

end type
}

procedure main {
    Create game structure taking in user defined options or using defaults, this calls on the game new() method
    game.render()

end procedure
}

Game method {
    function new(game_options): Self {
        return Self {
            cells: self.gen_cells_vec.make_cells_vec(game_options),
            game_options,
            active: false,
            player_count: null,
            players: null,
            top_player: null,
            messages: self.fetch_messages.fetch_messages()
        }

    end function
    }

end method
}

function make_cells_vec(game_options): Vector of Cells {
    var x = game_options.board_size
    var cells_vec = Vector with capacity of x * x

    for(var i = 0; i > x * x; x++) {
        if (((x - 1) - i / x) % 2 == 1 && x % 2 == 1) || (((x - 1) - 1 / x) % 2 == 0 && x % 2 == 0) {
            var x_val = (x - 1) - i % x
        } else {
            x_val = i % x

        end if
        }

        var new_cell = Cell {
            cell_number: i + 1,
            x: x_val,
            y: x - 1 - i / x,
            start: false,
            end: null,
        }
        push new_cell to cells_vec
    
    end loop
    }

    var obstacles = gen_obstacles(x, game_options)
    for each obstacle assing it to the corresponding cell in cells_vec

    return cells_vec

end function
}

function gen_obstacles(x, game_options): Vector of tuples made from 2 ints {
    var normal = Normal based on gaussian curve from 0 to x * 2
    var obs_vec = vector with capacity x * x
    for(var _i = 0; i =< x * 2; i++) {
        if game_options.random_obstacles == false {
            break from loop
        }

        do {
            var cell = generate range from x to x * x
            var variation = generated value using normal, rounded to an int from float

        } while (
            (cell + v <= 0)
            && (cell + v >= x * x - 1)
            && (v == 0)
            && (cell == x * x - 1)
            && (previous obstacles point to start of this one)
        )

        obs_vec.push((cell, variation))

    end loop
    }

    var user_obstacles = fetch_obstacles(x * x)
    append user_obstacles to obs_vec
    return obs_vec

end function
}

function fetch_obstacles(x): Vector of tuples made from 2 ints {
    var obstacle_vec = Vector
    if read_lines("assets/obstacles.txt") is ok {
        for line in lines from read_lines function {
            if line is ok {
            var regex = /\s*//.+/
            if line is empty or matches regex {
                continue
            }

            split cell at "->" and parse resulting vec of it into 2 ints (vec should be 2 strings of numbers)
            if parse worked {
                if parsed_num[0] > x || parsed_num[1] > x {
                    print("Board doesn't contain value where '{}' starts or ends", cell)
                    continue

                end if
                }
                push (parsed_num[0] - 1, parsed_num[1] - parsed_num[1]) to obstacle_vec
            } else if parse errored {
                print("'{}' is not valid obstacle. Define obstacle using start->end", cell)

            end if
            }

            end if
            }

        end loop
        }

    end if
    }
    return obstacle_vec

end function
}

function read_lines(filename): either iterator of lines or error {
    var file = File::open(filename)
    if reading file errors {
        return error

    end if
    }
    return file lines as an iteratable buffer

end function
}

function fetch_messages(): Hashmap with string as key and value {
    var msg_map = new hashmap
    if read_lines("assets/messages.txt") is ok {
        for line in lines {
            if line is ok {
                if message is null or messages starts with "//" {
                    continue

                end if
                }
                split message a first ":"
                insert values before ":" as key to hasmap and value after as value

            end if
            }
 
        end loop
        }

    end if
    }
    return msg_map

end function
}

GameOption method {
    function default(): Self {
        return Self {
            board_size: 7,
            dice_sides: 6,
            random_obstacles: true,
        }

    end function
    }

end method
}

Game method {
    procedure render() {
        Create a window called "Game" which is 512x612 pixels
        
        Read fonts from assets folder
        
        var border = Border coloured black with a radius of 1.0
        var rect = Rectangle coloured DEFAULT, shaped like a square with variable border as border
        var text_to_render = Text couloured BLACK, with font size 25, and rounded
        
        var turn = 1
        While window is open {
            var window_size = Size of the window
            window_size.height = window_size.height - 50

            if Game is not active {
                get_input(
                    Game,
                    window,
                    window_size,
                    window events,
                    font,
                    text
                )
            } else if Game is active and Game.top_player hasn't won {
                render_board(
                    Game,
                    mutable reference to window,
                    window_size,
                    reference to window event,
                    font,
                    text_to_render,
                )
    
                render_players(
                    Game,
                    mutable reference to window,
                    window_size,
                    refence to window events,
                )

                turn = render_turn(
                    Game,
                    mutable reference to window,
                    window_size,
                    reference to window events,
                    text_to_render,
                    turn,
                )
            } else {
                render_winner(
                    Game,
                    mutable reference of window,
                    window_size,
                    window events,
                    font,
                    text_to_render,
                )

            end if
            }

        end while
        }

    end procedure
    }

end method
}


procedure get_input(game, window, window_size, e, font, text) {
    Render green background

    // TODO: change this to work with messages hashmap
    Render "How many players?", at:
        X: (window_size.width / 2.0) - 145.0,
        Y: ((window_size.height + 50.0) / 2.0) + 10.0

    Render "Press a number key between 2 and 4", at: 
        X: (window_size.width / 2.0) - 169.0,
        Y: ((window_size.height + 50.0) / 2.0) + 10.0

    Flush glyphs cache

    Wait for a keypress
    game init_keypress(keypress)

end procedure
}

Game method {
    procedure init_keypress(self, button) {
        match button and set player amount to number between 2 and 4
    }
}

end procedure
}

procedure render_board (game, window, window_size, e, rect, font, text) {
    Render blue background
    set text font size to 220 / game board size

    loop through cells in game {
        draw_cells(game, rect window_size, refernce to cell, context, graphics)
        draw_nums(game, text, window_size, refernce to cell, font, context, graphics)
        if cell has a start of an obstacle {
            draw_line(refernce to game, cell, window_size, context, graphics)
        
        end if
        }

    end loop
    }
    flush glyph cache

end procedure    
}

procedure draw_line (game, rect, window_size, current_cell, context, graphics) {
    var rect_size =
        X: current_cell.x * window_size.width / game.game_options.board_size,
        Y: current_cell.y * (window_size.height - 50) / game.game_options.board_size + 50,
        Size X: window_size.width / game.game_options.board_size,
        Size Y: (window_size.height - 50) / game.game_options.board_size,

    if cell number is even change the colour of cell to create chequered pattern

    render rectangle using rect_size

end procedure
}

procedure draw_nums (game, text, window_size, current_cell, font, context, graphics) {
    var cell_num = current_cell.cell_number.to_string()

    var transform =
        X: (current_cell.x + 0.3) * window_size.width / game.game_options.board_size,
        Y: 50 + window_size.height / game.game_options.board_size / 1.5
            + (current_cell.y + 0.1) * (window_size.height - 50) / game.game_options.board_size,

    render text at transform position

end procedure
}

procedure draw_line (game, current_cell, window_size, context, graphics) {
    if cell.cell_number > game.cells[cell.end].cell_number {
        var colour = red
        var offset = -10
    } else {
        var colour = green
        var offset = 10
    
    end if
    }

    // 315844 is 512x612 which is the default size of the window
    var line = 
        color: colour,
        radius: 1 + 11 / game.game_options.board_size *
            (window_size.width * window_size.height / 315844),
        shape: Round,

    var shadow = same as line but transparent black color

    var line_pos = get_arrow_pos(game, cell, window_size, offset)
    var shadow_pos = get_arrow_pos(game, cell, window_size, offset + 5)

    render line and shadow

end procedure
}

// Defines the coordinates of the arrow as [hx, hy, tx, ty] where h is head and t is tail
// the arrows are offset to not overlap with each other
function get_arrow_pos (game, cell, window_size, offset):[array of floats with length of 4] {
    return [
        (cell.x + 0.5) * window_size.window_size / game.game_options.board_size,
        50 + window_size.height / game.game_options.board_size / 2
            + cell.y * (window_size.height - 50) / game.game_options.board_size
            + offset,
        (game.cells[cell.end].x + 0.5) * window_size.width
            / game.game_options.board_size,
        50 + window_size.height / game.game_options.board_size / 2
            + game.cells[cell.end].y * (window_size.height - 50)
            / game.game_options.board_size
            + offset,
    ]

end function
}

procedure render_players (game, window, window_size, event) {
    for cells in game.players {
        if player is on a cell {
            var counter_size = (window_size.height + window_size.width) / 2
                / game.game_options.board_size * 6

            var cell_x = cell.x * window_size.width / game.game_options.board_size
            var cell_y = cell.y * (window_size.height - 50) / game.game_options.board_size + 50

            match player.number {
                case 1: {
                    draw_circle(
                        [1.0, 0.0, 1.0, 1.0],
                        counter_size,
                        cell_x + 5,
                        cell_y + 5,
                        context,
                        graphics
                    )
                }
                case 2: {
                    draw_circle(
                        [0.0, 1.0, 1.0, 1.0],
                        counter_size,
                        cell_x + window_size.width / game.game_options.board_size - counter_size - 5,
                        cell_y + 5,
                        context,
                        graphics
                    )
                }
                case 3: {
                    draw_circle(
                        [0.0, 0.0, 1.0, 1.0],
                        counter_size,
                        cell_x + 5,
                        cell_y + (window_size.height - 50) / game.game_options.board_size - counter_size - 5,
                        context,
                        graphics
                    )
                }
                case 4: {
                    draw_circle(
                        [1.0, 1.0, 0.0, 1.0],
                        counter_size,
                        cell_x + (window_size.width - 50) / game.game_options.board_size - counter_size - 5,
                        cell_y + (window_size.height - 50) / game.game_options.board_size - counter_size - 5,
                        context,
                        graphics
                    )
                }
                case default: {
                    // Can't happen
                }

            end match
            }

        end if
        }
    
    end loop
    }

end procedure
}

procedure draw_circle (colour, size, x_pos, y_pos, context, graphics) {
    create a circle with colour

    var counter_pos = [
        x_pos, y_pos, size, size
    ]

    render the circle

end procedure
}

function render_turn (game, window, window_size, event, font, text, turn): int {
    var new_turn = turn
    if spacebar is pressed {
        var result = game.turn_keypress(button, turn)
        if result {
            new_turn = result
        end if
        }
    end if
    }

    text.font_size = 14
    var top_text = fetch message from messages hashmap and replace "{player}" with the current player number
    var text_transform =
        X: window_size.width / 2 - 227,
        Y: 33
    render top_text

    var prev_player = 
        if turn == 1 {
            game.player_count - 1
        } else {
            turn - 2 

        end if
        }

    var player = game.players[prev_player]
    if player rolled dice {
        var roll_data = player.roll
        text.font_size = 10
        var bottom_text = fetch message from messages hashmap
            replace "{player}" with player.number
            replace "{roll_one}" with roll_data.roll_one
            replace "{roll_two}" with roll_data.roll_two
            replace "{total}" with roll_data.roll_total
            replace "{start}" with roll_data.old_score
            replace "{end}" with roll_data.new_score

        var bottom_transform = 
            X: window_size.width / 2 - 236,
            Y: window_size.height + 13
        render bottom text

    end if
    }

    if player was on an obstacle {
        var obstacles = roll_data.obstacles
        for (iter, obstacle) in obstacles {
            var obstacle_text = fetch message from messages hashmap
                replace "{start}" with obstacle.start
                replace "{obstacle}" with obstacle.obstacle
                replace "{player}" with player.number
                replace "{end}" with obstacle.end
            
            var obstacle_transform = 
                X: window_size.width / 2 - 215,
                Y: window_size.height + (2 + iter) * 13

            render text

        end loop
        }

    end if
    }

    return new_turn

end function
}

Game method {
    function turn_keypress (self, button, player_num) : optional int {
        if button pressed is space {
            self.player_turn(player_num)
            if player_num == self.player_count {
                return 1
            } else {
                return player_num + 1

            end if
            }
        } else {
            return null

        end if
        }
   
    end function
    }

    procedure player_turn (self, player) {
        var player_vec = clone of self.players
        var player = player_vec[player_num - 1]
        var old_score = player.score
        var roll = roll_dice(self.game_options.dice_sides)
        var new_score = update_pos(player, roll[2], self)
        var obstacle_data = check_obstacle(player, self, new vector)
        var obstacles = obstacle_data if obstacles else null
        
        if no self.top_player or player.score > self.top_player.score {
            self.top_player = clone of player

        end if
        }

        player.roll = RollData strcture constructed with the variables defined above

    end procedure
    }

end method
}


function roll_dice (dice_sides): Vector of ints {
    var return_vec = vector with lenght of 3
    var roll_1 = random value with range 1 to dice sides
    var roll_2 = random value with range 1 to dice sides

    add roll_1 and roll_2 to return_vec

    // -1 because spec says 2 same numbers should move player backwards
    if roll_1 == roll_2 {
        var roll_total = -1 * (roll_1 + roll_2)
    } else {
        var roll_total = roll_1 + roll_2

    end if
    }
    push roll_total to return_vec
    return return_vec

end function
}

function update_pos (player, roll_total, game): int {
    player.score = player.score + roll_total or 0 if that's negative
    if player score is above 0 and the player hasn't won {
        player.cell = game.cells[player.score - 1]
        return player.score
    } else {
        player.cell = null
        return 0

    end if
    }

end function
}

function check_obstacle (player, game, obstacle_vec): Vector of ObstacleData {
    if player is on the board {
        var target_cell = player.cell
        if target_cell.start == true {
            update_pos(player, target_cell.end + 1 - target_cell.cell_number, game)
            var obstacle = 
                if target_cell.end + 1 - target_cell.cell_number > 0 {
                    Ladder
                } else {
                    Snake

                end if
                }

            obstacle_vec.push(ObstacleData{
                obstacle: obstacle,
                start: target_cell.cell_number,
                end: target_cell.end + 1
            })

            obstacle_vec = check_obstacle(player, game, obstacle_vec)

        end if
        }

    end if
    }

    return obstacle_vec

end function
}

procedure render_winner(game, window, window_size, event, font, text) {
    render white background
    read winner text from hashmap and reaplce "{player}" with game.top_player.number

    var text_transform = 
        X: window_size.width / 2 - 117,
        Y: (window_size.height + 50) / 2 - 15

    render text
  
end procedure
}
```
