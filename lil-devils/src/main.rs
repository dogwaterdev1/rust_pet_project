use std::io;

fn main() {
    println!("SELECT CHOICE");
    println!("INPUT A GUESS:");

    // get a guess, create a new tring
    let mut guess = String::new();

    // Used to read the guess and output an error if thrown
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line. Try an acceptable format...");

    println!("Guess Inputted: {guess}");
}
