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
        println!("You immediately say 'nope' and fuck off right back out the door");
        process::exit(exitcode::OK);
    } else {
        println!(
            "Lets get this over with. Have to end this. You walk on over to him and sit down."
        );
    }

    // store the choice selected from the decision.
    // decision 2
    chapter_1::decision_2();

    // // call guessing game
    let guessing_game_success: u32 = guessing_game::guessing_game_fn();

    if guessing_game_success >= 1 {
        println!("\nYou shake violently as you step out. That did not go as planned. Vomiting over the railing from the andrenal release. You gather your bearings, put the bag in the car, and leave. ");
    } else if guessing_game_success <= 0 {
        println!("\nYou stare blankly as he draws the barrel of his .45 to meet your eyes. You blink. You break into a cold sweat as your knees weaken and collapse. You beg, pleading for a second chance. He returns a weak, sad smile.\n\t 'no man can walk out on his own story, and yours ends here my friend. I'm sorry.'.\n You continue begging for your life as the tears sting your cheeks through your blurry, panicked vision. \n\t 'close your eyes.'\n you furiously shake your head. but before you can process his reaction you see a flash. a ringing in your ears.\n\n It all goes dark.. ");
    }
}
