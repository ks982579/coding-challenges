use std::env;
use std::path::PathBuf;
use std::fs::read_to_string;

mod garden;
use crate::garden::*;

fn main() {
    let mut puzzle_path: PathBuf = env::current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let puzzle: String = read_to_string(puzzle_path).unwrap();
    let mut the_maps: Mappings = Mappings::from_str(&puzzle);
    dbg!(the_maps);
}