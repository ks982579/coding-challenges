#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scratcher {
    id: usize,
    winners: Vec<usize>,
    numbers: Vec<usize>,
    points: usize,
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
}