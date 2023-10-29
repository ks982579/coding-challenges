pub mod humans {
    #[derive(Default, Debug, Clone)]
    pub enum AttackTypes {
        Physical,
        Energy,
        Fire,
        Electric,
        Ice,
    }

    trait Fighter {
        fn attack<T> (&self, opponent: T)
        where: T: Fighter,
        {
            println!("Attacking");
        }
    }

    pub struct BasicAttack {
        pub attack_types: Vec<AttackTypes>,
        pub damage: u32,
        pub effects: Option<&str>,
    }

    pub struct Human {
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

use crate::humans::Human;

fn game_match(fighter1: Human, fighter2: fighter) -> Human {

}

fn main() {
    println!("Hello, world!");
    let batman: Human = Human {
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