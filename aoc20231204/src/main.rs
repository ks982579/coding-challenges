mod scratcher;

use crate::scratcher::*;

use std::cmp::min;
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

    // Stuff all into a vec (usual)
    let mut all_scratchers: Vec<Scratcher> = Vec::new();

    for line in puzzle.lines() {
        let mut tmp_card = Scratcher::from_str(line);
        tmp_card.check_winnings();
        all_scratchers.push(tmp_card);
    }
    /* We hav all cards and counts. 
     * Need to instance  
     */
    let maximum: usize = all_scratchers.len();
    for x in (0..maximum) {
        // Scratcher is a small struct, so cloning is ok... for bigger data structures
        // consider cloning only what you need...
        let current_card: Scratcher = all_scratchers[x].clone();

        for y in (x+1..min(maximum, x+1+current_card.count)) {
            if let Some(card_to_update) = all_scratchers.get_mut(y) {
                card_to_update.instance += current_card.instance;
            }
        }
    }
    total = all_scratchers.iter().fold(
        0, |acc, b| acc + b.instance,
    );
    println!("Part 2: {}", total);
}
