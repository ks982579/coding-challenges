#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Race {
    pub time: u64,
    pub dist: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strategy {
    pub button_press: u64,
    pub speed: u64,
    pub dist: u64,
}

impl Race {
    pub fn from_row_str(row_str: &str) -> Vec<Race> {
        // First row is time
        let mut data_iter = row_str.lines();
        let mut time_str: &str = data_iter.next().unwrap();
        let mut dist_str: &str = data_iter.next().unwrap();

        time_str = time_str.split(":").collect::<Vec<&str>>()[1];
        dist_str = dist_str.split(":").collect::<Vec<&str>>()[1];


        let tmp_time_str: Vec<&str> = time_str.split_whitespace().collect();
        let times: Vec<u64> = tmp_time_str.into_iter().map(
            |x| x.parse::<u64>().unwrap()
        ).collect();
        let tmp_dest_str: Vec<&str> = dist_str.split_whitespace().collect();
        let dists: Vec<u64> = tmp_dest_str.into_iter().map(
            |x| x.parse::<u64>().unwrap()
        ).collect();
        let race_data: Vec<(&u64, u64)> = times.iter().zip(dists).collect();

        let mut races: Vec<Race> = Vec::with_capacity(5);
        for each_race in race_data {
            races.push(
                Race {
                    time: *each_race.0,
                    dist: each_race.1,
                }
            )
        }
        races
    }
    
    pub fn from_str_with_bad_kerning(row_str: &str) -> Race {
        // First row is time
        let mut data_iter = row_str.lines();
        let mut time_str: &str = data_iter.next().unwrap();
        let mut dist_str: &str = data_iter.next().unwrap();

        time_str = time_str.split(":").collect::<Vec<&str>>()[1];
        dist_str = dist_str.split(":").collect::<Vec<&str>>()[1];

        let tmp_time_str: Vec<&str> = time_str.split_whitespace().collect();
        let mut big_time: String = String::new();
        for lil_time in tmp_time_str.into_iter() {
            big_time.push_str(lil_time);
        }        
        
        let tmp_dest_str: Vec<&str> = dist_str.split_whitespace().collect();
        let mut big_dist: String = String::new();
        for lil_dist in tmp_dest_str.into_iter() {
            big_dist.push_str(lil_dist);
        }
        let utime: u64 = big_time.parse::<u64>().unwrap();
        let udist: u64 = big_dist.parse::<u64>().unwrap();
        Race { time: utime, dist: udist }
    }

    pub fn possible_winners(&self) -> Vec<Strategy> {
        /* 
        * Loop 0 -> Time + 1
        * strat::from_button_press
        * Check if strategy can win
        *    if yes - push into return vector
        *    else - discard
        * return
         */
        let mut winning_strategies: Vec<Strategy> = Vec::new();
        for i in (0..self.time + 1) {
            let button_press: u64 = self.time - i;
            let speed: u64 = button_press.clone();
            let distance: u64 = (self.time - button_press)*speed;
            if distance > self.dist {
                winning_strategies.push(
                    Strategy {
                        button_press: button_press,
                        speed: speed,
                        dist: distance,
                    }
                )
            }
        }
        winning_strategies
    }

    pub fn possible_winning_strategies(&self) -> u64 {
        /* 
        * Loop 0 -> Time + 1
        * strat::from_button_press
        * Check if strategy can win
        *    if yes - push into return vector
        *    else - discard
        * return
         */
        let mut winning_count: u64 = 0;
        let time: u64 = self.time as u64;
        let record: u64 = self.dist as u64;
        for i in (0..time + 1) {
            let button_press: u64 = time - i;
            let speed: u64 = button_press.clone();
            let distance: u64 = (time - button_press)*speed;
            if distance > record {
                // Idea is to find first forwards,
                // then start on the end and find first backwards
                // and that range is the number
                // But i'll try to brute force count
                winning_count += 1;
            }
        }
        winning_count
    }

}
