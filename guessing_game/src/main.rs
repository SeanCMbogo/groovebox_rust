use std::io;

fn main() {
    println!("Guess the number!");
    println!("Input your guess.");

    let apples = 5; // immutable
    let mut guess = String::new(); // mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
