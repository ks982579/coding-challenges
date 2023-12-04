mod scratcher;

use crate::scratcher::*;

use std::path::PathBuf;
use std::env::current_dir;
use std::fs::read_to_string;

fn main() {
    let mut puzzle_path: PathBuf = current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let mut puzzle = read_to_string(puzzle_path).unwrap();
    // let mut all_scratchers: Vec<Scratcher> = Vec::new();
    let mut total: usize = 0;
    for line in puzzle.lines() {
        // all_scratchers.push(
        //     Scratcher::from_str(line)
        // );
        let mut tmp_card = Scratcher::from_str(line);
        tmp_card.check_winnings();
        total += tmp_card.points;
    }
    println!("Part 1: {}", total);
}
