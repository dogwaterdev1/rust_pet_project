use std::collections::HashMap;
use std::env;
use std::fs;

/// .
/// Splits on ----- and returns the first half of the iterator
// TODO: Consolidate code into a function, starting to violate DRY
pub fn read_file_text(path: &String) -> (String) {
    let content = fs::read_to_string(path).expect("Failed to read text from file");
    let mut split_text = content.split("-----");
    // iterate over and get desired text
    let mut desired_text = "";
    for line in split_text {
        desired_text = line;
        break;
    }

    return String::from(desired_text);
}

pub fn read_file_choices(path: &String) -> (HashMap<u32, String>) {
    let content = fs::read_to_string(path).expect("Failed to read text from file");
    let mut split_text = content.split("-----");
    // iterate over and get desired text
    let mut choices_text = "";
    for line in split_text {
        choices_text = line;
    }

    // create a choices map
    let mut choices = HashMap::new();

    // create temporary variable and add items to choices map
    let mut counter: u32 = 1;
    let mut choices_text_parsed = choices_text.split("\n");
    for line in choices_text_parsed {
        // parse out line carriages
        if (line.starts_with("\r")) {
            continue;
        }
        choices.insert(counter, String::from(line));
        counter += 1;
    }

    return choices;
}

pub fn current_dir() {
    println!("{}", std::env::current_dir().unwrap().display());
}

// TODO: get user input
pub fn get_user_input() {}
