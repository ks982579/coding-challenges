use std::io;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};

/**
 * https://rust-random.github.io/book/
 * 
 *  */

struct Answer {
    val_str: String,
    val_int: i32,
}

// Because it mutates self, you must declare it as mutable
impl Answer {
    fn converter(&mut self) -> () {
        // TODO: catch non-int or it panics
        // TODO: Trim answers...
        self.val_int = self.val_str.parse::<i32>().unwrap();
    }
}

struct NumberRange {
    upper_bound: i32,
    lower_bound: i32,
}

impl NumberRange {
    fn increase_bounds(&mut self) -> () {
        self.upper_bound = self.upper_bound + 1;
        if (self.upper_bound - self.lower_bound) > 20 {
            self.lower_bound = self.lower_bound + 1;
        }
    }
    fn decrease_bounds(&mut self) -> () {
        self.upper_bound = self.upper_bound - 1;
        self.lower_bound = self.lower_bound - 1;
        if self.upper_bound < 10 {
            self.upper_bound = 10;
        }
        if self.lower_bound < 0 {
            self.lower_bound = 0;
        }
    }
}

fn main() {
    println!("This is my maths adding game!");
    println!("Enter any non-diget character to cause the program to panic to exit...");
    // Make random things
    let mut rng: ThreadRng = rand::thread_rng();
    let uniform: Uniform<i32> = Uniform::new(0, 10);
    let num_l: i32 = uniform.sample(&mut rng);
    let num_r: i32 = uniform.sample(&mut rng);
    let expected: i32 = num_l + num_r;
    println!("{num_l} + {num_r} = ?");


    let thing: String = get_answer();
    let mut answer: Answer = Answer {
        val_str: thing, // moved into here
        val_int: 0,
    };
    answer.converter();
    // let thing: i32 = thing.parse::<i32>().unwrap();

    // println!("{:?}", &answer.val_int);
    if answer.val_int == expected {
        println!("Correct!")
    } else {
        println!("Incorrect!")
    }
}

fn get_answer() -> String {
    let mut buffer: String = String::new();
    let stdin: io::Stdin = io::stdin(); // creating `Stdin` here

    // pass in reference and unwrap
    match stdin.read_line(&mut buffer) {
        Ok(_) => {
            buffer.pop(); // pops off the \n
            return buffer;
        }
        Err(error) => {
            println!("Error: {error}");
            return String::from("Sorry, error in the code");
        }
    }
}
