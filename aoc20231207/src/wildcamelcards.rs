use std::cmp::Ordering;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WildHand<'l> {
    pub cards: &'l str,
    pub card_counts: Vec<(char, u64)>,
    pub strength: Vec<u8>,
    pub bid: u64,
    pub rank: u64,
}

/* Shouldn't Change */
impl<'l> PartialOrd for WildHand<'l> {
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

impl<'l> Ord for WildHand<'l> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ordering) => ordering,
            None => Ordering::Equal,
        }
    }
}

fn wild_card_counter(cards: &str, card_counts: &mut Vec<(char, u64)>) {
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

    card_counts.sort_by(|a,b| b.1.cmp(&a.1));
    /* Get the Jokers */
    let mut joker_index: Option<usize> = None;
    for card_count in card_counts.iter().enumerate() {
        // (index, (card, count))
        if card_count.1.0 == 'J' {
            joker_index = Some(card_count.0);
        }
    }
    match joker_index {
        None => (),
        Some(index) => {
            if card_counts[index].1 < 5 {
                    let jokers = card_counts.remove(index);
                    card_counts[0].1 += jokers.1;
                }
        }
    }
    
}

/*
 * Word of Caution: The checks will not be mutually exclusive because they
 * are meant to be executed in order. That means the is_one_pair() will return
 * true for is_two_pair() and so on...
 */
impl<'l> WildHand<'l> {
    pub fn from_str(data: &'l str) -> WildHand<'l> {
        let data_split: Vec<&str> = data.trim().split_whitespace().collect::<Vec<&str>>();
        let cards_in_hand: &str = data_split[0];
        let current_bid: u64 = data_split[1].parse::<u64>().unwrap();
        let mut strengths: Vec<u8> = Vec::new();
        let mut card_counts: Vec<(char, u64)> = Vec::new();
        
        for card in cards_in_hand.chars() {
            if card == 'A' {
                strengths.push(14);
            } else if card == 'K' {
                strengths.push(13);
            } else if card == 'Q' {
                strengths.push(12);
            } else if card == 'J' {
                strengths.push(1);
            } else if card == 'T' {
                strengths.push(10);
            } else {
                strengths.push(
                    card.to_digit(10).unwrap() as u8
                )
            }
        }
        wild_card_counter(&cards_in_hand, &mut card_counts);
        WildHand {
            cards: cards_in_hand,
            card_counts: card_counts,
            strength: strengths,
            bid: current_bid,
            rank: 0,
        }
    }

    pub fn is_five_of_kind(&self) -> bool {

        if self.card_counts.len() == 1 {
            return true;
        }
       false
    }
    pub fn is_four_of_kind(&self) -> bool {
        // (card, count)
        
        if self.card_counts.len() == 2 {
            if self.card_counts[0].1 == 4 && self.card_counts[1].1 == 1 {
                return true;
            }
        }
        false
    }
    
    pub fn is_full_house(&self) -> bool {
        // (card, count)

        if self.card_counts.len() == 2 {
            if self.card_counts[0].1 == 3 && self.card_counts[1].1 == 2 {
                return true;
            }
        }
        false
    }

    pub fn is_three_of_kind(&self) -> bool {
        // (card, count)

        if self.card_counts.len() > 2 {
            if self.card_counts[0].1 == 3 {
                return true;
            }
        }
        false
    }

    pub fn is_two_pair(&self) -> bool {
        // (card, count)

        if self.card_counts.len() > 2 {
            if self.card_counts[0].1 == 2 && self.card_counts[1].1 == 2 {
                return true;
            }
        }
        false
    }

    pub fn is_one_pair(&self) -> bool {
        // (card, count)

        if self.card_counts.len() > 3 {
            if self.card_counts[0].1 == 2 {
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
        let expected: WildHand = WildHand {
            cards: "AKQJT",
            card_counts: vec![('A', 2), ('K', 1), ('Q', 1), ('T', 1)],
            strength: vec![14,13,12,11,10],
            bid: 10,
            rank: 0,
        };
        let actual: WildHand = WildHand::from_str(data);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_hand_is_5_of_kind() {
        let data: &str = "JJJJK 0";
        let hand = WildHand::from_str(data);
        assert!(hand.is_five_of_kind());
    }
    #[test]
    fn test_hand_is_4_of_kind() {
        let data: &str = "KTTTT 0";
        let hand = WildHand::from_str(data);
        assert!(hand.is_four_of_kind());
        let data: &str = "TTJTQ 0";
        let hand = WildHand::from_str(data);
        assert!(hand.is_four_of_kind());
        let data: &str = "JTJTQ 0";
        let hand = WildHand::from_str(data);
        assert!(hand.is_four_of_kind());
    }
    #[test]
    fn test_hand_is_full_house() {
        let data: &str = "KTTTK 0";
        let hand = WildHand::from_str(data);
        assert!(hand.is_full_house());
        let data: &str = "TTQJQ 0";
        let hand = WildHand::from_str(data);
        assert!(hand.is_full_house());
    }
    #[test]
    fn test_hand_is_three_of_kind() {
        let data: &str = "KTTTQ 0";
        let hand = WildHand::from_str(data);
        assert!(hand.is_three_of_kind());
        let data: &str = "5T5JQ 0";
        let hand = WildHand::from_str(data);
        assert!(hand.is_three_of_kind());
    }

    #[test]
    fn test_hand_is_two_pair() {
        let data1: &str = "KTQTQ 0";
        let hand1 = WildHand::from_str(data1);
        dbg!(&hand1);
        assert!(hand1.is_two_pair());
        // let data2: &str = "559AJ 0";
        // let hand2 = WildHand::from_str(data2);
        // dbg!(&hand2);
        // assert!(hand2.is_two_pair());
    }
    #[test]
    fn test_wild_card_counter() {
        let data: &str = "AAAAJ";
        let mut cc: Vec<(char, u64)> = Vec::new();
        wild_card_counter(data, &mut cc);
        assert_eq!(cc[0].1, 5);
    }
    #[test]
    fn test_sorting_manually() {
        let data1: &str = "A2345 0";
        let hand1 = WildHand::from_str(data1);
        let data2: &str = "2345A 0";
        let hand2 = WildHand::from_str(data2);
        let data3: &str = "34567 0";
        let hand3 = WildHand::from_str(data3);
        let data4: &str = "36547 0";
        let hand4 = WildHand::from_str(data4);
        let mut this = vec![data3, data2,data1,data4];
        this.sort();
        this.reverse();
        assert!(true);
    }
}