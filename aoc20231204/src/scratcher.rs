#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scratcher {
    pub id: usize,
    pub winners: Vec<usize>,
    pub numbers: Vec<usize>,
    pub points: usize,
}

impl Scratcher {
    pub fn from_str(line: &str) -> Scratcher{
        let split_colon: Vec<&str> = line.trim().split(':').collect();
        let mut card_id_str = split_colon[0];
        let numbers = split_colon[1];

        // Get Card ID
        let card_id: usize = card_id_str.trim().split_whitespace().filter_map(
            |s| s.parse::<usize>().ok()
        ).collect::<Vec<usize>>()[0];

        // Split numbers
        let numbers_split: Vec<&str> = numbers.trim().split('|').collect();
        let winners_str: &str = numbers_split[0];
        let candidates_str: &str = numbers_split[1];

        // Get Winners
        let winners: Vec<usize> = winners_str.trim()
            .split_whitespace().filter_map(
                |s| s.parse::<usize>().ok()
            ).collect();

        // Get Candidates
        let candidates: Vec<usize> = candidates_str.trim()
            .split_whitespace().filter_map(
                |s| s.parse::<usize>().ok()
            ).collect();
        
        Scratcher {
            id: card_id,
            winners: winners,
            numbers: candidates,
            points: 0,
        }
    }

    pub fn check_winnings(&mut self) {
        // reset points just in case
        self.points = 0;
        for n in self.winners.iter() {
            if self.numbers.contains(n) {
                if self.points == 0 {
                    self.points = 1;
                } else {
                    self.points *= 2;
                }
            }
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::Scratcher;

    #[test]
    fn test_test() {
        assert!(true);
    }
    #[test]
    fn test_create_scratcher_from_str() {
        let expected: Scratcher = Scratcher {
            id: 42,
            winners: vec![1,2,3,4,5],
            numbers: vec![1,2,3,4,5,6,7],
            points: 0,
        };
        let actual: Scratcher = Scratcher::from_str(
            "Card  42: 1 2  3  4  5 | 1 2 3  4  5  6  7"
        );
        assert_eq!(actual, expected);

        let wrong: Scratcher = Scratcher {
            id: 42,
            winners: vec![1,2,3,4,5],
            numbers: vec![1,2,3,4,5,9,7],
            points: 0,
        }; 
        assert_ne!(wrong, expected);
    }
    #[test]
    fn test_check_points() {
        let mut check1 = Scratcher::from_str(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
        );
        check1.check_winnings();
        let mut check2 = Scratcher::from_str(
            "Card  4: 41 92 73 84 69 | 41 92 73 84 69  1  2  3  4  5"
        );
        check2.check_winnings();
        let mut check3 = Scratcher::from_str(
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        );
        check3.check_winnings();
        assert_eq!(check1.points, 8);
        assert_eq!(check2.points, 16);
        assert_eq!(check3.points, 0);
    }
}