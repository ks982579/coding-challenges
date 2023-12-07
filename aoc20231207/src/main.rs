use std::env::current_dir;
use std::path::PathBuf;
use std::fs::read_to_string;

mod camelcards;
use camelcards::*;

fn main() {
    let mut puzzle_path: PathBuf = current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let puzzle: String = read_to_string(puzzle_path).unwrap();
    for hand in puzzle.lines() {
        todo!();
    }
}
