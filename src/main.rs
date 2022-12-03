use rand::{thread_rng, Rng};
use std::io;

// will be improved with Chapter 2 of rust docs
fn main() {
    let goal_num = thread_rng().gen_range(0..100) as u32;
    let mut guessed = String::new();
    println!("Enter a number to guess!");
    let mut num = 101;
    while num != goal_num {
        match io::stdin().read_line(&mut guessed) {
            Ok(_n) => {
                num = guessed.trim().parse::<u32>().unwrap();
                if goal_num < num {
                    println!("you guessed higher than target")
                } else if goal_num > num {
                    println!("you guessed lower than target")
                }
                guessed = String::new();
            }
            Err(error) => println!("{error}"),
        }
    }
    println!("Win!");
}
