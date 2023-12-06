mod boat;

use crate::boat::*;
use std::env::current_dir;
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() {
    println!("Hello, world!");
    let mut puzzle_path: PathBuf = current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let puzzle: String = read_to_string(puzzle_path).unwrap();
    for line in puzzle.lines() {
        println!("{}", line);
    }
    let mut races = Race::from_row_str(&puzzle);
    dbg!(&races);
    // flatten?
    /*
    * get vector of winning strategies
    *   flatten as needed
    * loop through
    *   multiply buttons for margin of error
    * print
     */
    let mut strategy_holder: Vec<Vec<Strategy>> = Vec::new();
    for race in races {
        strategy_holder.push(race.possible_winners())
    }
    // misunderstood question...
    // let winning_strategies: Vec<Strategy> = strategy_holder.into_iter().flatten().collect();

    let mut product: u64 = 1;
    for strat in strategy_holder.iter() {
        product *= strat.len() as u64;
        dbg!(&product);
    }
    println!("Margin of Error: {}", &product);
}
