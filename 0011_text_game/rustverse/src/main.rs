pub mod beings {
    pub struct Being {
        pub name: String,
        pub health: u32,
    }
}

use crate::beings::Being;

fn main() {
    println!("Hello, world!");
    let batman: Being = Being {
        name: String::from("Batman"),
        health: 100,
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_first() {
        assert!(true);
    }
}