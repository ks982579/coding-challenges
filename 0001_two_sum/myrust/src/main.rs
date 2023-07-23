struct Solution;

impl Solution {
    fn two_sum_double_loop(x: &[i32], y: &i32) -> bool {
        println!("Hello, world!");
        let list_length: usize = x.len();
        println!("{}", list_length);
        for i in 0..list_length {
            println!("{:?}.) {}", i, String::from("Is it working?"));
        }
        
        true
    }
}

fn main() {
    // println!("Hello, world!");
    let x: [i32; 6] = [1,2,3,4,5,6];
    let y: i32 = 5+6;
    Solution::two_sum_double_loop(&x, &y);
}

#[cfg(test)]
mod tests {

    use crate::Solution;

    #[test]
    fn test_sum_double_loop() {
        let test_list: [i32; 5] = [1,2,3,4,5];
        let test_sum: i32 = 4+5;
        assert_eq!(Solution::two_sum_double_loop(&test_list, &test_sum), true);
    }
}