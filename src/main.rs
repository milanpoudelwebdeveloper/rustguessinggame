use std::io;
fn main() {
    println!("Guess the number!");

    println!("Please input your guess!");

    //capture input from user
    //Create a variable
    //In rust, variables are immutable by default so we have to use "mut"
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to run line");
}
