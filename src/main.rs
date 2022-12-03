use std::io;

fn main() {
    println!("Enter a number to guess!");
    let mut guessed = String::new();
    match io::stdin().read_line(&mut guessed) {
        Ok(n) => {
            println!("{n} {guessed}");
        }
        Err(error) => println!("{error}"),
    }
}
