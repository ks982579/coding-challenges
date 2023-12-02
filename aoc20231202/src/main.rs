mod cube;

use crate::cube::{Game, Round};
use std::path::PathBuf;
use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    /* Part I Solution */
    // Reading in from file
    let mut puzzle_path: PathBuf = env::current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let puzzle: String = fs::read_to_string(puzzle_path)?;

    // Creating the max allowed cubes per round
    let max_round: Round = Round {
        red: 12,
        blue: 14,
        green: 13,
    };
    let mut possible_games: Vec<Game> = Vec::new();
    for game in puzzle.lines() {
        let txt: &str = game.trim();
        if txt == "" {
            continue;
        } else {
            let current_game: Game = Game::from_str(txt, Some(&max_round));
            if current_game.possible {
                possible_games.push(current_game);
            }
        }
    }
    // Summing possible IDs
    let folded: i32 = possible_games.iter().fold(
        0,
        |acc, e| acc + e.id
    );
    println!("Part 1: {}", folded);

    /* Part II Solution */
    let mut total_power: i32 = 0;
    for game in puzzle.lines() {
        let txt: &str = game.trim();
        if txt == "" {
            continue;
        } else {
            let current_game: Game = Game::from_str(txt, None);
            let min_set: Round = current_game.find_min_set();
            total_power += min_set.get_set_power();
        } 
    }
    println!("Total power: {}", total_power);
    
    Ok(())
}

