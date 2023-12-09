use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

mod state_machine;
use state_machine::*;

// Infallible - cool
fn main() {
    let path_str: Vec<String> = env::args().collect();
    let mut puzzle_path: PathBuf;
    if path_str.len() > 1 {
        puzzle_path = PathBuf::from_str(&path_str[1]).unwrap();
        puzzle_path.canonicalize().unwrap();
    } else {
        puzzle_path = env::current_dir().unwrap();
        puzzle_path.push("puzzle.txt");
    }
    let puzzle: String = read_to_string(puzzle_path).unwrap();
    let mut instructions: Instructions;
    for line_tup in puzzle.lines().enumerate() {
        if line_tup.0 == 0 {
            instructions = Instructions::from_str(line_tup.1);
        } else if line_tup.1.trim() != "" {
            todo!();
        }
    }
}
