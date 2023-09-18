use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("-*- Guessing Game -*-");
    

    let secret_number = rand::thread_rng().gen_range(1..=100);

    
    loop {
        println!("=> ");
        let mut guess_number = String::new();
        io::stdin().read_line(&mut guess_number).expect("Error! read_line");
        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!")
        }
    }
    
}
