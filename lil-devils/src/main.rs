// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
use std::collections::HashMap;

mod chapter_1;
mod guessing_game;

fn main() {

    // store the choice selected from the decision.
    chapter_1::decision_2();

    // call guessing game
    let guessing_game_success: u32 = guessing_game::guessing_game_fn();

    if guessing_game_success >= 1{
        println!("\nCongrats! you've won");
    }
    else if guessing_game_success <= 0 {
        println!("\nSorry, you lost the guessing game");

    }
}
