mod lib;//::elf_fun;

pub use crate::lib::elf_fun;
use std::path::PathBuf;
use std::fs;
use std::env;

fn main() {
    elf_fun::elf_call();
    let mut puzzle_path: PathBuf = env::current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let puzzle: String = fs::read_to_string(puzzle_path).unwrap();
    for line in puzzle.lines() {
        dbg!(line);
    }
}
