use std::io;

struct Answer {
    val_str: String,
    val_int: i32,
}

// Because it mutates self, you must declare it as mutable
impl Answer {
    fn converter(&mut self) -> () {
        self.val_int = self.val_str.parse::<i32>().unwrap();
    }
}

fn main() {
    println!("Hello, world!");
    let thing: String = get_answer();
    let mut answer: Answer = Answer {
        val_str: thing, // moved into here
        val_int: 0,
    };
    answer.converter();
    // let thing: i32 = thing.parse::<i32>().unwrap();

    println!("{:?}", &answer.val_int);
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
