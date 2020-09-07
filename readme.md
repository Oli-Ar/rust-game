# Rust Snakes and Ladders
Rust snakes and ladders is the classic game of snakes and ladders programmed in rust using the 
[piston_window crate](https://github.com/PistonDevelopers/piston_window). It allows users to add their own obstacles 
or use randomly generated obstacles, as well as allowing the user to change the size of the board. 
It was created as a school project.
### Compile Guide:
1. Install Rust using [rustup](https://rustup.rs/)
2. Go to directory with this project
3. Run the command `cargo run --release` to build and run the project (if you `cargo build --release` ensure that the 
file is in ./sixth_form_summer_work/ due to file paths for assests)
### Specification 
The game was made following the instructions from [this pdf](https://pdfhost.io/v/xBfvK9fub_project_specpdf.pdf).


## Plan
Game:
- Have a main game struct containing methods and other structs allowing the game to be rendered
and for the players to play the game.
- The game struct should contain a player and cell struct that contain information specifically on the 
players and each cell on the main game board.
- It should implement a method which allows the game to be rendered with the user defined option.
- The game board should allow users to manually enter in obstacles and/or randomly generate the obstacles.
- The user should be able to define: whether obstacles are randomly generated, where the obstacles are 
generated, how many users are playing, the size of the board, and how many sides the dice has.
