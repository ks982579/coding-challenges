enum Almanac {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temp,
    Humid,
    Location
}

#[derive(Default, Debug)]
pub struct Seed {
    pub seed: u64,
    pub soil: u64,
    pub fert: u64,
    pub water: u64,
    pub light: u64,
    pub temp: u64,
    pub humid: u64,
    pub location: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct InnerMap {
    pub destination_start: u64,
    pub source_start: u64,
    pub range_len: u64,
}

#[derive(Debug, Default, Clone)]
pub struct Mappings {
    pub seeds: Vec::<u64>,
    pub seed_to_soil: Vec::<InnerMap>,
    pub soil_to_fert: Vec::<InnerMap>,
    pub fert_to_water: Vec::<InnerMap>,
    pub water_to_light: Vec::<InnerMap>,
    pub light_to_temp: Vec::<InnerMap>,
    pub temp_to_humid: Vec::<InnerMap>,
    pub humid_to_location: Vec::<InnerMap>,
}

impl Mappings {
    pub fn from_str(data: &str, is_range: bool) -> Self {
        let mut state: Almanac = Almanac::Seed;

        let mut mapping: Mappings = Mappings::default();

        for lin in data.lines() {
            match state {
                Almanac::Seed => {
                    match is_range {
                        true => {
                            let mut s_mem: Option<u64> = None;
                            let str_seeds: Vec::<&str> = lin.trim()
                                .split(":").collect::<Vec<&str>>()[1]
                                .split_whitespace().collect();
                            for each_seed in str_seeds {
                                let seed_num: u64 = each_seed.parse::<u64>().unwrap();
                                match s_mem {
                                    None => {
                                        s_mem = Some(seed_num);
                                    }
                                    Some(init_seed) => {
                                        for i in (init_seed..init_seed+seed_num) {
                                            mapping.seeds.push(i);
                                        }
                                    }
                                }
                            }
                        }
                        false => {
                            if lin.trim() == "" {
                                state = Almanac::Soil;
                            } else {
                                let str_seeds: Vec::<&str> = lin.trim()
                                    .split(":").collect::<Vec<&str>>()[1]
                                    .split_whitespace().collect();
                                for seed in str_seeds {
                                    mapping.seeds.push(seed.parse::<u64>().unwrap());
                                }
                            }
                        }
                    }
                }
                Almanac::Soil => {
                    if lin.trim() == "" {
                        state = Almanac::Fertilizer;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.seed_to_soil.push(tmp_inner);
                    }
                }
                Almanac::Fertilizer => {
                    if lin.trim() == "" {
                        state = Almanac::Water;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.soil_to_fert.push(tmp_inner);
                    }
                }
                Almanac::Water => {
                    if lin.trim() == "" {
                        state = Almanac::Light;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.fert_to_water.push(tmp_inner);
                    }
                }
                Almanac::Light => {
                    if lin.trim() == "" {
                        state = Almanac::Temp;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.water_to_light.push(tmp_inner);
                    }
                }
                Almanac::Temp => {
                    if lin.trim() == "" {
                        state = Almanac::Humid;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.light_to_temp.push(tmp_inner);
                    }
                }
                Almanac::Humid => {
                    if lin.trim() == "" {
                        state = Almanac::Location;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.temp_to_humid.push(tmp_inner);
                    }
                }
                Almanac::Location => {
                    if lin.trim() == "" {
                        continue;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.humid_to_location.push(tmp_inner);
                    }
                }
                _ => {}
            }
        }
        mapping
    }
}


#[derive(Debug, Default, Clone)]
pub struct SeedlessMappings {
    pub seed_to_soil: Vec::<InnerMap>,
    pub soil_to_fert: Vec::<InnerMap>,
    pub fert_to_water: Vec::<InnerMap>,
    pub water_to_light: Vec::<InnerMap>,
    pub light_to_temp: Vec::<InnerMap>,
    pub temp_to_humid: Vec::<InnerMap>,
    pub humid_to_location: Vec::<InnerMap>,
}

impl SeedlessMappings {
    pub fn from_str(data: &str) -> Self {
        let mut state: Almanac = Almanac::Soil;

        let mut mapping: SeedlessMappings = SeedlessMappings::default();

        for lin in data.lines() {
            match state {
                Almanac::Soil => {
                    if lin.trim() == "" {
                        state = Almanac::Fertilizer;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.seed_to_soil.push(tmp_inner);
                    }
                }
                Almanac::Fertilizer => {
                    if lin.trim() == "" {
                        state = Almanac::Water;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.soil_to_fert.push(tmp_inner);
                    }
                }
                Almanac::Water => {
                    if lin.trim() == "" {
                        state = Almanac::Light;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.fert_to_water.push(tmp_inner);
                    }
                }
                Almanac::Light => {
                    if lin.trim() == "" {
                        state = Almanac::Temp;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.water_to_light.push(tmp_inner);
                    }
                }
                Almanac::Temp => {
                    if lin.trim() == "" {
                        state = Almanac::Humid;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.light_to_temp.push(tmp_inner);
                    }
                }
                Almanac::Humid => {
                    if lin.trim() == "" {
                        state = Almanac::Location;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.temp_to_humid.push(tmp_inner);
                    }
                }
                Almanac::Location => {
                    if lin.trim() == "" {
                        continue;
                    } else if lin.contains(':') {
                        continue;
                    } else {
                        let str_values: Vec<&str> = lin.trim().split_whitespace().collect();
                        let mut values: Vec<u64> = Vec::new();
                        for val in str_values {
                            values.push(val.parse::<u64>().unwrap());
                        }
                        let tmp_inner: InnerMap = InnerMap {
                            destination_start: values[0],
                            source_start: values[1],
                            range_len: values[2],
                        };
                        mapping.humid_to_location.push(tmp_inner);
                    }
                }
                _ => {}
            }
        }
        mapping
    }
}