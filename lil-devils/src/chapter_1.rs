use std::collections::HashMap;
use std::env;
use std::fs;

    pub fn decision_1() -> () {//u32 {
        // read the data associated with choice
        //let content: str = read_file("./chapter_1_text/decision_2.txt");
        let content = fs::read_to_string(r".\chapter_1_text\decision_1.txt").expect("Failed to read decision 1 text");

        println!("{:?}\n", &content);
        // map of choices, and the 
        let mut scores = HashMap::new();
    
        scores.insert(1, String::from("Blue"));
        scores.insert(2, String::from("Yellow"));
    
        let team_name :u32 = 2;
        //let score = scores.get(&team_name).copied().unwrap_or(0);
        let score = scores.get(&team_name);//.copied();
        println!("{:?}",&score);
        
    }

    pub fn decision_2() -> (){
        // read the data associated with choice
        //let content: str = read_file("./chapter_1_text/decision_2.txt");
        let content = fs::read_to_string(r".\chapter_1_text\decision_2.txt").expect("Failed to read decision 2 text");
        println!("{:?}\n", &content);

        // map of choices, and the 
        let mut scores = HashMap::new();
        scores.insert(1, String::from("New York City"));
        scores.insert(2, String::from("Las Vegas"));
    
        let team_name :u32 = 2;
        //let score = scores.get(&team_name).copied().unwrap_or(0);
        let score = scores.get(&team_name);//.copied();
        println!("{:?}",&score);
    }

