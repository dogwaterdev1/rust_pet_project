use crate::helper_functions;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::option;

/// First decision text, meant to be called from the main menu
pub fn decision_1() -> (u32) {
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

/// Second decision of the program, meant to be called from the main menu
pub fn decision_2() -> (u32) {
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
