use std::env::current_dir;
use std::path::PathBuf;
use std::fs::read_to_string;

mod engine;
use engine::*;

fn main() {
    println!("Hello, world!");
    let mut puzzle_path: PathBuf = current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let schematics: String = read_to_string(puzzle_path).unwrap();
    let mut ai: MemoryCell = MemoryCell::default();
    let mut total: u32 = 0;
    let mut length: usize = 0;
    for line in schematics.lines() {
        if length == 0 {
            length = line.len();
        }
        ai.remember(line.trim());
        let parts: Vec<u32> = ai.find_engine_parts();
        for part in parts {
            total += part;
        }
    }
    // Case of last line...
    let last_memory: &str = &".".repeat(length);
    ai.remember(last_memory);
    let parts = ai.find_engine_parts();
    for part in parts {
        total += part;
    }
    println!("Part 1: {}", total);
}

