#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Race {
    pub time: u32,
    pub dist: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Strategy {
    pub button_press: u32,
    pub speed: u32,
    pub dist: u32,
}

impl Race {
    pub fn from_row_str(row_str: &str) -> Vec<Race> {
        // First row is time
        let mut data_iter = row_str.lines();
        let mut time_str: &str = data_iter.next().unwrap();
        let mut dist_str: &str = data_iter.next().unwrap();
        dbg!(&time_str);
        dbg!(&dist_str);

        time_str = time_str.split(":").collect::<Vec<&str>>()[1];
        dist_str = dist_str.split(":").collect::<Vec<&str>>()[1];

        dbg!(&time_str);
        dbg!(&dist_str);

        let tmp_time_str: Vec<&str> = time_str.split_whitespace().collect();
        let times: Vec<u32> = tmp_time_str.into_iter().map(
            |x| x.parse::<u32>().unwrap()
        ).collect();
        dbg!(&times);
        let tmp_dest_str: Vec<&str> = dist_str.split_whitespace().collect();
        let dists: Vec<u32> = tmp_dest_str.into_iter().map(
            |x| x.parse::<u32>().unwrap()
        ).collect();
        dbg!(&dists);
        let race_data: Vec<(&u32, u32)> = times.iter().zip(dists).collect();
        dbg!(&race_data);

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
            let button_press: u32 = self.time - i;
            let speed: u32 = button_press.clone();
            let distance: u32 = (self.time - button_press)*speed;
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
}
