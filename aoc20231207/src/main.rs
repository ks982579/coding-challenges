use std::env::current_dir;
use std::path::PathBuf;
use std::fs::read_to_string;

mod camelcards;
use camelcards::*;

/*
sting splits to hand and bid

*/
fn main() {
    let mut puzzle_path: PathBuf = current_dir().unwrap();
    puzzle_path.push("puzzle.txt");
    let puzzle: String = read_to_string(puzzle_path).unwrap();
    let mut five_kind: Vec<Hand> = Vec::new();
    let mut four_kind: Vec<Hand> = Vec::new();
    let mut full_house: Vec<Hand> = Vec::new();
    let mut three_kind: Vec<Hand> = Vec::new();
    let mut two_pair: Vec<Hand> = Vec::new();
    let mut one_pair: Vec<Hand> = Vec::new();
    let mut high_card: Vec<Hand> = Vec::new();
    for hand in puzzle.lines() {
        let tmp_hand = Hand::from_str(hand);
        if tmp_hand.is_five_of_kind() {
            five_kind.push(tmp_hand);
        } else if tmp_hand.is_four_of_kind() {
            four_kind.push(tmp_hand);
        } else if tmp_hand.is_full_house() {
            full_house.push(tmp_hand);
        } else if tmp_hand.is_three_of_kind() {
            three_kind.push(tmp_hand);
        } else if tmp_hand.is_two_pair() {
            two_pair.push(tmp_hand);
        } else if tmp_hand.is_one_pair() {
            one_pair.push(tmp_hand);
        } else {
            high_card.push(tmp_hand);
        }
    }
    high_card.sort();
    for this in high_card.iter() {
        dbg!(&this.cards);
    }
    one_pair.sort();
    two_pair.sort();
    three_kind.sort();
    full_house.sort();
    four_kind.sort();
    five_kind.sort(); 
    // let mut five_of_kinds: Vec<Hand> = Vec
    let mut all_hands = vec![
        high_card,
        one_pair,
        two_pair,
        three_kind,
        full_house,
        four_kind,
        five_kind,
    ];
    let flat_hands: Vec<Hand> = all_hands.into_iter().flatten().collect();
    // dbg!(&flat_hands);

    // dbg!(&flat_hands.len());
    let mut winnings: u64 = 0;
    let mut count: u64 = 1;
    for hand in flat_hands.iter() {
        let current_win = hand.bid*count;
        winnings += current_win;
        count += 1;
        // println!("{}", &hand.cards);
        // println!("{} * {} = {}", &hand.bid, &count, &current_win);
        // println!("{}", &winnings);
        // println!("---------");
    }
    println!("Part I: {}", winnings);
}
