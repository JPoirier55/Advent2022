use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    //day 6 1
    // day6(contents, 4);
    // day 6 2
    day6(contents, 14);
}

fn day6(contents: String, length: usize) {
    let mut char2: Vec<char> = contents.chars().collect::<Vec<_>>();
    let mut char3: Vec<char> = Vec::new();

    let mut count = 0;
    let mut winner = false;
    for i in length..char2.len()-1 {
        let mut char3: Vec<char> = Vec::new();
        for m in 0..length {
            char3.push(char2[i-m]);
        }
        char3.sort_by(|a, b| b.cmp(a));
        let mut final_vec: Vec<char> = Vec::new();
        for c in 0..char3.len()-1 {
            if char3[c] != char3[c+1] {
                final_vec.push(char3[c]);                
            } 
        }
        if final_vec.len() == length-1 {
            println!("{}", i+1);
            return;
        }
    }
}