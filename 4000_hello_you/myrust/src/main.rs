use std::io::stdin;

fn main() {
    let mut line: String = String::new();
    println!("What is your name?");
    let _bytes_read: usize = stdin().read_line(&mut line).unwrap();
    println!("Hello, {}!", line.trim());
}
