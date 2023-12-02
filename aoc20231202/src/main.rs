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
    let mut max_round: Round = Round {
        red: 12,
        blue: 14,
        green: 13,
    };
    let mut possible_games: Vec<Game> = Vec::new();
    for game in puzzle.lines() {
        dbg!(&game);
        let txt: &str = game.trim();
        if txt == "" {
            continue;
        } else {
            let current_game: Game = Game::from_str(txt, Some(&max_round));
            if current_game.possible {
                println!("Possible => {:?}", &current_game);
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


    
    Ok(())
}

