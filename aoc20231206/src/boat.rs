pub struct Race {
    pub time: u32,
    pub dist: u32,
}

impl Race {
    pub fn from_row_str(row_str: &str) {
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

        let mut times: Vec<u32> = Vec::new();
        let mut dists: Vec<u32> = Vec::new();

        let tmp_time_str = time_str.replace("\t", " ");
        let times_x: Vec<&str> = tmp_time_str.split_whitespace().collect();
        dbg!(&times_x);
    }
}
