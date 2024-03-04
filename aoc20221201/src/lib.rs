use std::ops::{Add, Sub, Mul, Div};
use std::marker::Sized;

trait Number: Add + Sub + Mul + Div + Sized {}

#[derive(Debug, Clone, )]
struct Elf<T> 
where
    T: Number,
{
    snacks: Vec<T>,
    total_cals: T,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_working(){
        assert_eq!(1,1);
    }
    // #[test]
}