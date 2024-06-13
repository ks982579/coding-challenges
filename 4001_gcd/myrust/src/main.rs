// Rust prelude might take care of trait imports
use std::env;
use std::str::FromStr;

fn gcd(n1: i32, n2: i32) -> Option<i32> {
    let mut div: i32 = if n1 < n2 { n1 } else { n2 };
    loop {
        if div <= 0 {
            break;
        } else if (n1 % div) == 0 && (n2 % div) == 0 {
            break;
        } else {
            div -= 1;
        }
    }
    if div > 0 {
        Some(div)
    } else {
        None
    }
}

fn strictly_positive_ints(nums: &[i32]) -> Result<(), String> {
    for val in nums.iter() {
        if *val <= 0 as i32 {
            return Err(format!("{} not strictly positive", val));
        }
    }
    Ok(())
}

fn main() {
    let clargs: Vec<String> = env::args().collect();
    let num1: i32 = i32::from_str(&clargs[1]).expect("Could not convert value into integer.");
    let num2: i32 = i32::from_str(&clargs[2]).expect("Could not convert value into integer.");
    let _ = strictly_positive_ints(&[num1, num2]).unwrap();
    let num3 = gcd(num1, num2);
    match num3 {
        Some(n) => println!("GCD = {}", n),
        _ => println!("No GCD... something wrong probably happened."),
    }
}
