mod boat;

use crate::boat::*;
use std::env::current_dir;
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() {
    println!("Hello, world!");
    let mut puzzle_path: PathBuf = current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let puzzle: String = read_to_string(puzzle_path).unwrap();
    for line in puzzle.lines() {
        println!("{}", line);
    }
    Race::from_row_str(&puzzle);
}
