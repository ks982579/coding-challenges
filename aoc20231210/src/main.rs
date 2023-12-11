use std::env;
use std::path::PathBuf;
use std::fs::read_to_string;

mod maze_runner;
use crate::maze_runner::*;


fn main() {
    let mut puzzle_path = env::current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let puzzle = read_to_string(puzzle_path).unwrap();
    println!("printing puzzle?");
    let mut  maze = PipeMaze::from_str(&puzzle);
    // println!("{:?}", &maze);
    let mut wierd_animal = MazeRunner::default();
    // println!("{:?}", wierd_animal);
    wierd_animal.traverse_maze(&mut maze);
    println!("total steps in loop: {}", &wierd_animal.steps);
    println!("Half way: {}", &wierd_animal.steps / 2);
    let count = wierd_animal.count_nest_spots(&maze);
    println!("{}", count);
    
}
