pub mod beings {
    #[derive(Default, Debug, Clong)]
    pub enum AttackTypes {
        physical,
        fire,
        electric,
        ice,
    }

    pub struct BasicAttack {
        pub attack_types: Vec<AttackTypes>,
        pub damage: u32,
        pub effects: Optional<str>,
    }

    pub struct Being {
        pub name: String,
        pub health: u32,
        pub strength: u32,
    }
}

/**
 * url: https://stackoverflow.com/questions/19650265/is-there-a-faster-shorter-way-to-initialize-variables-in-a-rust-struct
 * You can implement Default trait for struct
 * Then use spread syntax 
 */

use crate::beings::Being;

fn main() {
    println!("Hello, world!");
    let batman: Being = Being {
        name: String::from("Batman"),
        health: 100,
        strength: 10,
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_first() {
        assert!(true);
    }
}