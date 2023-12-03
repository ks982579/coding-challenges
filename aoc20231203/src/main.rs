use std::env::current_dir;
use std::path::PathBuf;
use std::fs::read_to_string;

mod engine;
// use engine;

fn main() {
    println!("Hello, world!");
    let mut puzzle_path: PathBuf = current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let schematics: String = read_to_string(puzzle_path).unwrap();
    for line in schematics.lines() {
        dbg!(line);
        println!("I love my wife!")
    }
}

