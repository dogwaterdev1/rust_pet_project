extern crate exitcode;
use std::{collections::HashMap, process};
mod chapter_1;
mod guessing_game;
mod helper_functions;

/// main program to run everything.
fn main() {
    // decision 1
    let choice_1 = chapter_1::decision_1();
    // specific actions based upon what was inputted
    if choice_1 == 2 {
        println!("You immediately say 'nope' and walk right back out the door");
        process::exit(exitcode::OK);
    }

    // store the choice selected from the decision.
    // decision 2
    chapter_1::decision_2();

    // // call guessing game
    let guessing_game_success: u32 = guessing_game::guessing_game_fn();

    if guessing_game_success >= 1 {
        println!("\nYou joyfully step out, and leave the area. ");
    } else if guessing_game_success <= 0 {
        println!("\nYou lost. Next round is on you.");
    }
}
