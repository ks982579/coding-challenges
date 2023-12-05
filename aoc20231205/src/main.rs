use std::env;
use std::path::PathBuf;
use std::fs::read_to_string;

mod garden;
use crate::garden::*;

fn main() {
    let mut puzzle_path: PathBuf = env::current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let puzzle: String = read_to_string(puzzle_path).unwrap();
    let mut the_maps: Mappings = Mappings::from_str(&puzzle);
    
    let mut all_seeds: Vec<Seed> = Vec::new();

    for seed in the_maps.seeds {
        let mut tmp_seed: Seed = Seed::default();
        tmp_seed.seed = seed;

        // Could derive Iterable on the struct...

        let mut update = 0;
        
        for im in the_maps.seed_to_soil.clone() {
            if im.source_start <= tmp_seed.seed && tmp_seed.seed < im.source_start + im.range_len {
                let x = tmp_seed.seed - im.source_start;
                tmp_seed.soil = im.destination_start + x;
                update = 1;
            }
        }
        if update == 1 {
            update = 0;
        } else {
            tmp_seed.soil = tmp_seed.seed.clone();
        }

        for im in the_maps.soil_to_fert.clone() {
            if im.source_start <= tmp_seed.soil && tmp_seed.soil < im.source_start + im.range_len {
                let x = tmp_seed.soil - im.source_start;
                tmp_seed.fert = im.destination_start + x;
                update = 1;
            }
        }
        if update == 1 {
            update = 0;
        } else {
            tmp_seed.fert = tmp_seed.soil.clone();
        }

        for im in the_maps.fert_to_water.clone() {
            if im.source_start <= tmp_seed.fert && tmp_seed.fert < im.source_start + im.range_len {
                let x = tmp_seed.fert - im.source_start;
                tmp_seed.water = im.destination_start + x;
                update = 1;
            }
        }
        if update == 1 {
            update = 0;
        } else {
            tmp_seed.water = tmp_seed.fert.clone();
        }

        for im in the_maps.water_to_light.clone() {
            if im.source_start <= tmp_seed.water && tmp_seed.water < im.source_start + im.range_len {
                let x = tmp_seed.water - im.source_start;
                tmp_seed.light = im.destination_start + x;
                update = 1;
            }
        }
        if update == 1 {
            update = 0;
        } else {
            tmp_seed.light = tmp_seed.water.clone();
        }

        for im in the_maps.light_to_temp.clone() {
            if im.source_start <= tmp_seed.light && tmp_seed.light < im.source_start + im.range_len {
                let x = tmp_seed.light - im.source_start;
                tmp_seed.temp = im.destination_start + x;
                update = 1;
            }
        }
        if update == 1 {
            update = 0;
        } else {
            tmp_seed.temp = tmp_seed.light.clone();
        }

        for im in the_maps.temp_to_humid.clone() {
            if im.source_start <= tmp_seed.temp && tmp_seed.temp < im.source_start + im.range_len {
                let x = tmp_seed.temp - im.source_start;
                tmp_seed.humid = im.destination_start + x;
                update = 1;
            }
        }
        if update == 1 {
            update = 0;
        } else {
            tmp_seed.humid = tmp_seed.temp.clone();
        }

        for im in the_maps.humid_to_location.clone() {
            if im.source_start <= tmp_seed.humid && tmp_seed.humid < im.source_start + im.range_len {
                let x = tmp_seed.humid - im.source_start;
                tmp_seed.location = im.destination_start + x;
                update = 1;
            }
        }
        if update == 1 {
            update = 0;
        } else {
            tmp_seed.location = tmp_seed.humid.clone();
        }
        all_seeds.push(tmp_seed);
    }
    let mut smallest_location: u64 = u64::MAX;
    for seed in all_seeds {
        if seed.location < smallest_location {
            smallest_location = seed.location;
        }
    }
    println!("Part 1: {}", smallest_location);
}