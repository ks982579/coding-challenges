use std::cmp::Ordering;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Hand<'l> {
    pub cards: &'l str,
    pub strength: Vec<u8>,
    pub bid: u64,
    pub rank: u64,
}

impl<'l> PartialOrd for Hand<'l> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for i in (0..self.strength.len()) {
            let cmp: Option<Ordering> = self.strength[i].partial_cmp(&other.strength[i]);
            match cmp {
                Some(ordering) => {
                    match ordering {
                        Ordering::Greater => return Some(ordering),
                        Ordering::Less => return Some(ordering),
                        Ordering::Equal => continue,
                    }
                }
                None => continue,
            }
        }
        None
    }
}

impl<'l> Ord for Hand<'l> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ordering) => ordering,
            None => Ordering::Equal,
        }
    }
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
        let mut strengths: Vec<u8> = Vec::new();
        
        for card in cards_in_hand.chars() {
            if card == 'A' {
                strengths.push(14);
            } else if card == 'K' {
                strengths.push(13);
            } else if card == 'Q' {
                strengths.push(12);
            } else if card == 'J' {
                strengths.push(11);
            } else if card == 'T' {
                strengths.push(10);
            } else {
                strengths.push(
                    card.to_digit(10).unwrap() as u8
                )
            }
        }
        Hand {
            cards: cards_in_hand,
            strength: strengths,
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

    pub fn is_three_of_kind(&self) -> bool {
        // (card, count)
        let mut cc: Vec<(char, u64)> = Vec::with_capacity(5);
        
        card_counter(self.cards, &mut cc);
        cc.sort_by(|a,b| b.1.cmp(&a.1));

        if cc.len() > 2 {
            if cc[0].1 == 3 {
                return true;
            }
        }
        false
    }

    pub fn is_two_pair(&self) -> bool {
        // (card, count)
        let mut cc: Vec<(char, u64)> = Vec::with_capacity(5);
        
        card_counter(self.cards, &mut cc);
        cc.sort_by(|a,b| b.1.cmp(&a.1));

        if cc.len() > 2 {
            if cc[0].1 == 2 && cc[1].1 == 2 {
                return true;
            }
        }
        false
    }

    pub fn is_one_pair(&self) -> bool {
        // (card, count)
        let mut cc: Vec<(char, u64)> = Vec::with_capacity(5);
        
        card_counter(self.cards, &mut cc);
        cc.sort_by(|a,b| b.1.cmp(&a.1));

        if cc.len() > 3 {
            if cc[0].1 == 2 {
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
            strength: vec![14,13,12,11,10],
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
    #[test]
    fn test_hand_is_three_of_kind() {
        let data: &str = "KTTTQ 0";
        let hand = Hand::from_str(data);
        assert!(hand.is_three_of_kind());
        let data: &str = "5T55Q 0";
        let hand = Hand::from_str(data);
        assert!(hand.is_three_of_kind());
    }

    #[test]
    fn test_hand_is_two_pair() {
        let data: &str = "KTQTQ 0";
        let hand = Hand::from_str(data);
        assert!(hand.is_two_pair());
        let data: &str = "559AA 0";
        let hand = Hand::from_str(data);
        assert!(hand.is_two_pair());
    }
    #[test]
    fn test_sorting_manually() {
        let data1: &str = "A2345 0";
        let hand1 = Hand::from_str(data1);
        let data2: &str = "2345A 0";
        let hand2 = Hand::from_str(data2);
        let data3: &str = "34567 0";
        let hand3 = Hand::from_str(data3);
        let data4: &str = "36547 0";
        let hand4 = Hand::from_str(data4);
        let mut this = vec![data3, data2,data1,data4];
        this.sort();
        this.reverse();
        dbg!(this);
        assert!(false);
    }
}