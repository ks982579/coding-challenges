use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::thread;

mod garden;
use crate::garden::*;

fn main() {
    let mut puzzle_path: PathBuf = env::current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let puzzle: String = read_to_string(puzzle_path).unwrap();
    let mut the_maps: Mappings = Mappings::from_str(&puzzle, false);

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
            if im.source_start <= tmp_seed.water && tmp_seed.water < im.source_start + im.range_len
            {
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
            if im.source_start <= tmp_seed.light && tmp_seed.light < im.source_start + im.range_len
            {
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
            if im.source_start <= tmp_seed.humid && tmp_seed.humid < im.source_start + im.range_len
            {
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
    /* ---------------------------------------------- */

    // reset
    let mut puzzle_vec_seeds = puzzle.split("\n").collect::<Vec<&str>>();
    let mut puzzle_str_seeds = puzzle_vec_seeds[0].split(":").collect::<Vec<&str>>()[1];
    let mut seed_pairs: Vec<(u64, u64)> = Vec::new();
    let mut seed_numbers: Vec<&str> = puzzle_str_seeds.split_whitespace().collect();
    let mut memory: Option<u64> = None;
    smallest_location = u64::MAX;

    let mut seedless_puzzle: String = String::new();
    for (index, line) in puzzle.lines().enumerate() {
        if index > 1 {
            seedless_puzzle.push_str(line);
            seedless_puzzle.push_str("\n");
        }
    }
    let mut seedless_mapping: SeedlessMappings = SeedlessMappings::from_str(&seedless_puzzle);

    dbg!(&seedless_mapping);
    // Concurrent programming would be nice...
    for sd in seed_numbers {
        let seed_val: u64 = sd.parse::<u64>().unwrap();
        match memory {
            None => memory = Some(seed_val),
            Some(init_seed) => {
                println!("Starting at seed {:#}", &init_seed);
                println!("Ending as seed {:#}", init_seed+seed_val+1);
                for x in (init_seed..init_seed+seed_val+1) {
                    smallest_location = find_closest(x, &seedless_mapping, smallest_location)
                }
                println!("local minimum: {}", &smallest_location);
                memory = None;
            }
        }
    }
    println!("Part 2: {}", smallest_location);
}

fn find_closest(seed_val: u64, the_maps: &SeedlessMappings, small_val: u64) -> u64 {
    let mut tmp_seed: Seed = Seed::default();
    tmp_seed.seed = seed_val;


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
        if im.source_start <= tmp_seed.water && tmp_seed.water < im.source_start + im.range_len
        {
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
        if im.source_start <= tmp_seed.light && tmp_seed.light < im.source_start + im.range_len
        {
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
        if im.source_start <= tmp_seed.humid && tmp_seed.humid < im.source_start + im.range_len
        {
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


    if tmp_seed.location < small_val {
        return tmp_seed.location.clone();
    } else {
        return small_val;
    }
}
