use std::fs;
use std::path::Path;
use std::env;

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
            let digits: Vec<i32> = pull_digits(l);
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
