use std::env;
use std::fs;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    // A for Rock, B for Paper, and C for Scissors
    // X for Rock, Y for Paper, and Z for Scissors
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents: Vec<String> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    
    // day2_1(contents);
    day2_2(contents);
}

fn day2_1(contents: Vec<String>) {
    // 1 for Rock, 2 for Paper, and 3 for Scissors (what you draw)
    // 0 if you lost, 3 if the round was a draw, and 6 if you won
    let mut total_score: i32 = 0;
    for game in contents {
        if game != "" {
            let my_hand_score = hand_score(game.split(" ").collect::<Vec<&str>>()[1]);
            let result: i32 = does_my_hand_win(game.replace(" ", ""));
            total_score = total_score + my_hand_score + result;
        }
    }
    println!("{}", total_score);
}

fn day2_2(contents: Vec<String>) {
    // 1 for Rock, 2 for Paper, and 3 for Scissors (what you draw)
    // 0 if you lost, 3 if the round was a draw, and 6 if you won
    // X means you need to lose
    // Y means you need to end the round in a draw 
    // Z means you need to win
    let mut total_score: i32 = 0;
    for game in contents {
        if game != "" {
            let opp_hand = game.split(" ").collect::<Vec<&str>>()[0];
            let result_code = game.split(" ").collect::<Vec<&str>>()[1];
            let result = result_score(result_code, opp_hand);
            total_score = total_score + result;
        }
    }
    println!("{}", total_score);
}

fn hand_score(my_hand: &str) -> i32 {
    match my_hand{
        "X"=>return 1,
        "Y"=>return 2,
        "Z"=>return 3,
        "A"=>return 1,
        "B"=>return 2,
        "C"=>return 3,
        _=>return 0
    }
}

fn result_score(my_hand: &str, opp_hand: &str) -> i32 {
    match my_hand{
        "X"=>return 0 + losing(opp_hand),
        "Y"=>return 3 + hand_score(opp_hand),
        "Z"=>return 6 + winning(opp_hand),
        _=>return 0
    }
}

fn losing(hand: &str) -> i32 {
    match hand{
        "A"=>return 3,
        "B"=>return 1,
        "C"=>return 2,
        _=>return 0
    }
}
fn winning(hand: &str) -> i32 {
    match hand{
        "A"=>return 2,
        "B"=>return 3,
        "C"=>return 1,
        _=>return 0
    }
}

fn does_my_hand_win(game: String) -> i32 {
    let winning = vec_of_strings!["AY", "BZ", "CX"];
    let losing = vec_of_strings!["AZ", "BX", "CY"];
    let tie = vec_of_strings!["AX", "BY", "CZ"];
    if winning.iter().any(|i| i==&game) {
        return 6;
    }
    if losing.iter().any(|i| i==&game) {
        return 0;
    }
    if tie.iter().any(|i| i==&game) {
        return 3;
    }
    return 0;
}
