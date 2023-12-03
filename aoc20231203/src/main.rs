use std::env::current_dir;
use std::path::PathBuf;
use std::fs::read_to_string;

mod engine;
use engine::*;

fn main() {
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

    // Starting Part 2 with reset
    ai = MemoryCell::default();
    total = 0;
    length = 0;
    let mut potential_gears: Vec<GearPart> = Vec::new();
    for line in schematics.lines() {
        if length == 0 {
            length = line.len();
        }
        ai.remember(line.trim());
        let parts: Vec<GearPart> = ai.find_gear_parts();
        for part in parts {
            potential_gears.push(part);
        }
        // for part in parts {
        //     total += part;
        // }
        ai.vert_line += 1;
    }
    // Case of last line...
    let last_memory: &str = &".".repeat(length);
    ai.remember(last_memory);
    let parts = ai.find_gear_parts();
    ai.vert_line += 1;
    // for part in parts {
    //     total += part;
    // }
    for part in parts {
        potential_gears.push(part);
    }
    let mut tmp_gears: Vec<GearPart> = Vec::new();
    for potential_gear in potential_gears {
        for tgear in tmp_gears.iter() {
            if potential_gear.coordinate == tgear.coordinate {
                total += potential_gear.value.clone() * tgear.value.clone();
            }
        }
        tmp_gears.push(potential_gear);
    }
    println!("Part 2: {}", total);
}

