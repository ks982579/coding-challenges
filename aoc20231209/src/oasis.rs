#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Report {
    pub values: Vec<isize>,
    pub len: usize,
    pub first: isize,
    pub last: isize,
}

impl Report {
    pub fn from_vec(data: Vec<isize>) -> Report {
        let length = data.len();
        // dbg!(&length);
        let last_val = data[length-1].clone();
        let first_val: isize = data[0].clone();
        // dbg!(&data);
        Report {
            values: data,
            len: length,
            first: first_val,
            last: last_val,
        }
    }

    pub fn from_str(data: &str) -> Report {
        let bits: Vec<&str> = data.trim().split_whitespace().collect();
        let nbits: Vec<isize> = bits.into_iter().map(
            |x| x.trim().parse::<isize>().unwrap()
        ).collect();

        Report::from_vec(nbits)
    }

    pub fn is_zeros(&self) -> bool {
        self.values.iter().all(|x| 0 == *x)
    }

    pub fn find_next_value(&self) -> isize {
        let mut differences: Vec<isize> = Vec::with_capacity(self.values.len() -1);
        let mut memory: Option<isize> = None;
        for val in self.values.iter() {
            match memory {
                None => memory = Some(val.clone()),
                Some(prev_val) => {
                    let diff: isize = val - prev_val;
                    differences.push(diff);
                    memory = Some(val.clone());
                }
            }
        }

        let new_report = Report::from_vec(differences);
        let mut history: isize = 0;

        // something wrong here...
        if !new_report.is_zeros() {
            let extrap = new_report.find_next_value();
     
            history += extrap;
        }
        self.last.clone() + history
    }

    pub fn find_previous_value(&self) -> isize {
        let mut differences: Vec<isize> = Vec::with_capacity(self.values.len() -1);
        let mut memory: Option<isize> = None;
        for val in self.values.iter() {
            match memory {
                None => memory = Some(val.clone()),
                Some(prev_val) => {
                    let diff: isize = val - prev_val;
                    differences.push(diff);
                    memory = Some(val.clone());
                }
            }
        }

        let new_report = Report::from_vec(differences);
        let mut history: isize = 0;

        // something wrong here...
        if !new_report.is_zeros() {
            let extrap = new_report.find_previous_value();
            history += extrap;
        }
        self.first.clone() - history
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tests() {
        let this = "-42";
        let that = this.parse::<i32>().unwrap();
        assert_eq!(that, -42);
    }
    #[test]
    fn test_create_report_from_str() {
        let data = "1 2 -3 4 -42";
        let actual = Report::from_str(data);
        let expected = Report {
            values: vec![1,2,-3,4,-42],
            len: 5,
            first: 1,
            last: -42,
        };
        let fake = Report { values: vec![1,2,3], len: 3, first:1, last: 3};
        assert_eq!(actual, expected);
        assert_ne!(actual, fake);
    }

    #[test]
    fn test_finding_next_value() {
        let this = Report::from_str("10  13  16  21  30  45");
        let next_val = this.find_next_value();
        assert_eq!(next_val, 68);
    }

    #[test]
    fn test_finding_previous_value() {
        let this = Report::from_str("10  13  16  21  30  45");
        let prev_val = this.find_previous_value();
        assert_eq!(prev_val, 5);
    }
}