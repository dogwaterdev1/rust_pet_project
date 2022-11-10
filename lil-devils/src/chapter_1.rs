use crate::helper_functions;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::option;

// TODO: Consolidate code into a function, starting to violate DRY
pub fn decision_1() -> (u32) {
    // this is how you find the current directory in/calling from
    // println!("{}", std::env::current_dir().unwrap().display());
    // read the data associated with choice
    // let content = fs::read_to_string("./src/chapter_1_text/decision_1.txt")
    //     .expect("Failed to read decision 1 text");
    let filepath = String::from("./src/chapter_1_text/decision_1.txt");
    let content = helper_functions::read_file_text(&filepath);
    println!("{}", &content);
    let choices = helper_functions::read_file_choices(&filepath);

    for (key, value) in choices.iter() {
        println!("{}", value);
    }

    // Get User Input - write this helper in helper_functions.rs
    let option_selected = helper_functions::get_user_input(&choices);

    // print out selected options
    let chosen_action = choices
        .get(&option_selected)
        .expect("Could not retrieve value from Hashmap"); //.copied();
    println!("SELECTED OPTION: {}\n", chosen_action);

    return option_selected;
}

pub fn decision_2() -> (u32) {
    // have him say some dialogue and if you dont leave he holds you up and forces you to guess a number.
    // // read the data associated with choice
    // //let content: str = read_file("./chapter_1_text/decision_2.txt");
    // let content = fs::read_to_string("./src/chapter_1_text/decision_2.txt")
    //     .expect("Failed to read decision 2 text");

    // println!("{}\n", &content);
    // // map of choices, and the
    // let mut scores = HashMap::new();
    // scores.insert(1, String::from("New York City"));
    // scores.insert(2, String::from("Las Vegas"));
    // let team_name: u32 = 2;
    // //let score = scores.get(&team_name).copied().unwrap_or(0);
    // let score = scores
    //     .get(&team_name)
    //     .expect("Could not retrieve value from Hashmap"); //.copied();

    // println!("{}", score);

    let filepath = String::from("./src/chapter_1_text/decision_2.txt");
    let content = helper_functions::read_file_text(&filepath);
    println!("{}", &content);
    let choices = helper_functions::read_file_choices(&filepath);

    for (key, value) in choices.iter() {
        println!("{}", value);
    }

    // Get User Input - write this helper in helper_functions.rs
    let option_selected = helper_functions::get_user_input(&choices);

    // print out selected options
    let chosen_action = choices
        .get(&option_selected)
        .expect("Could not retrieve value from Hashmap"); //.copied();
    println!("SELECTED OPTION: {}\n", chosen_action);

    return option_selected;
}
