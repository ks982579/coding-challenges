mod cube;

// use crate::cube;
use std::path::PathBuf;
use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut puzzle_path: PathBuf = env::current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    dbg!(&puzzle_path);
    let puzzle: String = fs::read_to_string(puzzle_path)?;
    for game in puzzle.lines() {
        dbg!(game);
    }


    
    Ok(())
}

