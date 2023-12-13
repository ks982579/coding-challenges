use std::env;
use std::path::PathBuf;
use std::fs::read_to_string;
use std::str::FromStr;

mod cosmos;
use crate::cosmos::*;

fn main() -> Result<(), std::io::Error>{
    let mut puzzle_path: PathBuf = env::current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let puzzle: String = read_to_string(puzzle_path)?;
    let universe = Universe::from_str(&puzzle)?;


    Ok(())
}
