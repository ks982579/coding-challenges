#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand<'l> {
    pub cards: &'l str,
    pub bid: u64,
    pub rank: u64,
}

fn card_counter(cards: &str, card_counts: &mut Vec<(char, u64)>) {
    /* Function to count cards in hands and put into vector */
    for card in cards.chars() {
        if card_counts.len() < 1 {
            card_counts.push((card, 1));
        } else {
            let mut added: bool = false;
            for ctup in card_counts.clone().iter_mut().enumerate() {
                // (indes, (card, count))
                if card == ctup.1.0 {
                    card_counts[ctup.0].1 += 1;
                    added = true;
                    break;
                }
            }
            if !added {
                card_counts.push((card, 1));
            }
        }
    }
}

/*
 * Word of Caution: The checks will not be mutually exclusive because they
 * are meant to be executed in order. That means the is_one_pair() will return
 * true for is_two_pair() and so on...
 */
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

    pub fn is_five_of_kind(&self) -> bool {
        let mut mem: Option<char> = None;

        for card in self.cards.chars() {
            match mem {
                None => mem = Some(card),
                Some(prev_card) => {
                    if prev_card != card {
                        return false;
                    }
                }
            }
        }
        true
    }
    pub fn is_four_of_kind(&self) -> bool {
        // (card, count)
        let mut cc: Vec<(char, u64)> = Vec::with_capacity(5);
        
        card_counter(self.cards, &mut cc);
        cc.sort_by(|a,b| b.1.cmp(&a.1));
        if cc.len() == 2 {
            if cc[0].1 == 4 && cc[1].1 == 1 {
                return true;
            }
        }
        false
    }
    
    pub fn is_full_house(&self) -> bool {
        // (card, count)
        let mut cc: Vec<(char, u64)> = Vec::with_capacity(5);
        
        card_counter(self.cards, &mut cc);
        cc.sort_by(|a,b| b.1.cmp(&a.1));

        if cc.len() == 2 {
            if cc[0].1 == 3 && cc[1].1 == 2 {
                return true;
            }
        }
        false
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
    #[test]
    fn test_hand_is_5_of_kind() {
        let data: &str = "TTTTT 0";
        let hand = Hand::from_str(data);
        assert!(hand.is_five_of_kind());
    }
    #[test]
    fn test_hand_is_4_of_kind() {
        let data: &str = "KTTTT 0";
        let hand = Hand::from_str(data);
        assert!(hand.is_four_of_kind());
        let data: &str = "TTTTQ 0";
        let hand = Hand::from_str(data);
        assert!(hand.is_four_of_kind());
    }
    #[test]
    fn test_hand_is_full_house() {
        let data: &str = "KTTTK 0";
        let hand = Hand::from_str(data);
        assert!(hand.is_full_house());
        let data: &str = "TTQTQ 0";
        let hand = Hand::from_str(data);
        assert!(hand.is_full_house());
    }
}