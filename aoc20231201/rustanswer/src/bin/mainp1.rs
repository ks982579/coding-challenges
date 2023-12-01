fn main() {
    println!("Hello, world!");
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

#[cfg(test)]
mod tests {
    use crate::pull_digits;

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
}
