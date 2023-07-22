struct Solution;

impl Solution {
    fn two_sum_double_loop(x: &[i32]) -> bool {
        println!("Hello, world!");
        true
    }
}

fn main() {
    // println!("Hello, world!");
    Solution::two_sum_double_loop(&[1,2,3]);
}

#[cfg(test)]
mod tests {

    use crate::Solution;

    #[test]
    fn test_sum_double_loop() {
        assert_eq!(Solution::two_sum_double_loop([1,2,3]), true);
    }
}