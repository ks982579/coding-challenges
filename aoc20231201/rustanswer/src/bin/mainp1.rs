use std::fs;
use std::path::Path;
use std::env;

const SPELT_NUMBERS: [(&str, &str);10] = [
    ("zero", "0"),
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9")
];

fn main() {
    println!("Hello, world!");

    let puzzle_path: &Path = Path::new("./puzzle.txt");
    let puzzle_res: std::io::Result<String> = fs::read_to_string(puzzle_path);
    let puzzle: String = puzzle_res.unwrap();
    let mut calibrations: Vec<i32> = Vec::new();
    for l in puzzle.lines() {
        println!("{}", l);
        let trimmed: &str = l.trim();
        if trimmed == "" {
            continue;
        } else {
            let new_line: &str = &string_to_digit(l);
            let digits: Vec<i32> = pull_digits(new_line);
            println!("{:?}", digits);
            let calibration: i32 = two_digit_number(digits);
            println!("{:?}", calibration);
            calibrations.push(calibration);
        }
    }
    let mut total: i32 = 0;
    for cal in calibrations {
        total += cal;
    }
    println!("Total Calibration: {}", total);
}

fn string_to_digit(text: &str) -> String {
    println!("OK");
    let mut new_text: String = String::from(text.trim());
    for combo in SPELT_NUMBERS {
        new_text = new_text.replace(combo.0, combo.1);
    }
    new_text
}

fn pull_digits(text: &str) -> Vec<i32> {
    let mut calibration_candidates: Vec<i32> = Vec::new();
    for c in text.chars() {
        // Radix of 2 = binary, 10 = dec, 16 = hex
        if let Some(i) = c.to_digit(10) {
            calibration_candidates.push(i as i32);
        }
    }
    calibration_candidates
}

fn two_digit_number(numbers: Vec<i32>) -> i32 {
    let tens: i32 = numbers[0] * 10;
    let ones: i32 = numbers[numbers.len() -1];
    tens + ones
}

#[cfg(test)]
mod tests {
    use crate::{pull_digits, two_digit_number};

    #[test]
    fn it_works() {
        let result: i8 = 4;
        assert_eq!(result, 4);
    }
    #[test]
    fn vec_of_values() {
        let result: Vec<i32> = vec![1, 2, 3];
        let text: &str = "Ab1,=2/#3poi";
        let actual: Vec<i32> = pull_digits(text);
        assert_eq!(actual, result);
    }
    #[test]
    fn test_two_dig_numbers() {
        let digies = vec![4,5,3,6,2];
        let expected: i32 = 42;
        let actual: i32 = two_digit_number(digies);
        assert_eq!(actual, expected);
    }
}
