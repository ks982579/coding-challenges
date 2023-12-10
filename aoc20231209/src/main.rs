use std::env::current_dir;
use std::path::PathBuf;
use std::fs::read_to_string;

mod oasis;
use crate::oasis::*;

fn main() {
    println!("Hello, world!");
    let mut puzzle_path: PathBuf = current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let puzzle: String = read_to_string(puzzle_path).unwrap();

    let mut answer: isize = 0;
    for line in puzzle.lines() {
        let tmp_report = Report::from_str(line);
        answer += tmp_report.find_next_value();
    }

    println!("Part I: {}", answer);
}
