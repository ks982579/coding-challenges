pub mod fighters;

use fighters::Fighter;
use fighters::humans::Human;



/**
 * url: https://stackoverflow.com/questions/19650265/is-there-a-faster-shorter-way-to-initialize-variables-in-a-rust-struct
 * You can implement Default trait for struct
 * Then use spread syntax 
 */

fn game_match(fighter1: Human, fighter2: Human) -> Human {
    unimplemented!();
}

fn main() {
    println!("Hello, world!");
    let mut batman: Human = Human {
        name: String::from("Batman"),
        health: 100,
        strength: 10,
    };
    let mut tod: Human = Human {
        name: String::from("Tod"),
        health: 37,
        strength: 8,
    };

    // Game state need be updated all time so defeated don't attack also
    loop {
        dbg!(&batman);
        dbg!(&tod);
        batman.attack(&mut tod);
        tod.attack(&mut batman);

        if batman.health <= 0 {
            println!("{} is the victor!", &tod.name);
            break;
        }
        if tod.health <= 0 {
            println!("{} is the victor!", &batman.name);
            break;
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_first() {
        assert!(true);
    }
}