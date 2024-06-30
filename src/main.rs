use std::io;
//import "Rng" from rand package or dependency
use rand::Rng;
fn main() {
    println!("Guess the number!");

    println!("Please input your guess!");

    //capture input from user
    //Create a variable
    //In rust, variables are immutable by default so we have to use "mut"

    let mut rng = rand::thread_rng();

    let secret_number = rng.gen_range(1..101);

    println!("Secret number generated is {}", secret_number);
    //here new creates a new empty string
    let mut guess = String::new();

    //stdin() is a function here which provides here the utils like read line
    io::stdin()
        //read line here takes a buffer string. here "&mut" means that read line can  read user input and
        //can modify the guess variable with that input
        .read_line(&mut guess)
        .expect("Failed to run line");

    //here {} is a placeholder
    println!("You guessed: {}", guess)
}
