#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand<'l> {
    pub cards: &'l str,
    pub bid: u64,
    pub rank: u64,
}

impl<'l> Hand<'l> {
    pub fn from_str(data: &'l str) -> Hand<'l> {
        let data_split: Vec<&str> = data.trim().split_whitespace().collect::<Vec<&str>>();
        let cards_in_hand: &str = data_split[0];
        let current_bid: u64 = data_split[1].parse::<u64>().unwrap();
        Hand {
            cards: cards_in_hand,
            bid: current_bid,
            rank: 0,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tests() {
        assert!(true);
    }
    #[test]
    fn test_create_hand_from_str() {
        let data: &str = "AKQJT 10";
        let expected: Hand = Hand {
            cards: "AKQJT",
            bid: 10,
            rank: 0,
        };
        let actual: Hand = Hand::from_str(data);
        assert_eq!(actual, expected);
    }
}