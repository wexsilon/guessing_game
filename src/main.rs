use std::io::{self, Write};

fn main() {
    println!("-*- Guessing Game -*-");
    print!("=> ");

    io::stdout().flush().expect("Error! flush");

    let mut text_input = String::new();

    io::stdin().read_line(&mut text_input).expect("Error! read_line");

    

}
