/**
 * This is a game that asks a maths question and you provide solution.
 * It's in a loop, so you can work on those skills.
 * TODO:
 * [ ] - increase and decrease difficulty based on correct answers.
 * [ ] - implement other operations other than adding.
 * [ ] - Implement better logic to quit
 * [ ] - scoring
 * [ ] - Timer of some sort - going into multi-threading?
 */
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::io;

/**
 * https://rust-random.github.io/book/
 *
 */

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
    fn new() -> NumberRange {
        NumberRange {
            upper_bound: 10,
            lower_bound: 0,
        }
    }
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

enum Veracity {
    Correct,
    Incorrect,
}

fn main() {
    println!("This is my maths adding game!");
    println!("Enter any non-diget character to cause the program to panic to exit...");
    // Make random things
    let mut rng: ThreadRng = rand::thread_rng();

    let mut challenge: NumberRange = NumberRange::new();

    /*
    let uniform: Uniform<i32> = Uniform::new(challenge.lower_bound, challenge.upper_bound);

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
    let veracity: Veracity = check_answer(answer.val_int, expected);

    match veracity {
        Veracity::Correct => println!("Correct!"),
        Veracity::Incorrect => println!("Incorrect!"),
    }
    */

    loop {
        game_logic(&mut rng, &mut challenge);
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

fn check_answer(actual: i32, expected: i32) -> Veracity {
    if actual == expected {
        return Veracity::Correct;
    } else {
        return Veracity::Incorrect;
    }
}

fn game_logic(random_thread_range: &mut ThreadRng, challenge_set: &mut NumberRange) -> () {
    let uniform: Uniform<i32> = Uniform::new(challenge_set.lower_bound, challenge_set.upper_bound);

    /**
     * the sample method takes in a mutable reference of ThreadRng
     * Because this function already takes in that mutable reference,
     * just pass the argument into method.
     */
    let num_l: i32 = uniform.sample(random_thread_range);
    let num_r: i32 = uniform.sample(random_thread_range);
    let expected: i32 = num_l + num_r;
    println!("{num_l} + {num_r} = ?");

    let thing: String = get_answer();
    let mut answer: Answer = Answer {
        val_str: thing, // moved into here
        val_int: 0,
    };
    answer.converter();
    // let thing: i32 = thing.parse::<i32>().unwrap();
    let veracity: Veracity = check_answer(answer.val_int, expected);

    match veracity {
        Veracity::Correct => {
            println!("Correct!");
            challenge_set.increase_bounds();
        }
        Veracity::Incorrect => {
            println!("Incorrect!");
            challenge_set.decrease_bounds();
        }
    }
}
