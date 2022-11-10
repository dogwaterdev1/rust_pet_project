use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessing_game_fn() -> u32 {
    // create the max guessing counter
    let max_guesses: u32 = 10;
    // create a guessing counter
    let mut counter: u32 = 0;
    // Guess a number program minigame. based off of code from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
    println!(
        "'Well guess what. we're playing a game', He speaks as he prints his pistol through his shirt. \n\t'Guess the number and I'll let you live; you have only {} guesses to get it right'.",
        &max_guesses
    );
    // create the secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    while counter < max_guesses {
        println!("\nPlease input your guess.");
        // create a new string
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //compare the numbers and increment the counter, return 0 if it you don't guess it in the alloted guesses
        // return 1 if you guessed it within the alloted guessses
        println!("You guessed: {guess}");
        // print out the number of guesses left for the user. had to add 1, otherwise it gives wrong number of guesses left
        let mut guesses_left: u32 = max_guesses - (counter + 1);
        println!("Guesses left: {}", guesses_left);
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("\t'Wrong. Too small.'");
                counter = counter + 1;
            }
            Ordering::Greater => {
                println!("\t'Wrong. Too big.'");
                counter = counter + 1;
            }
            Ordering::Equal => {
                println!("\t'You win. You can leave.'");
                return 1;
            }
        }
    }
    println!("\t'You've lost.'");
    return 0;
}
