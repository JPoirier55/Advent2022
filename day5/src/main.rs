use std::env;
use std::fs;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    let mut warehouse: Vec<Vec<String>> = vec![vec_of_strings!["Z", "N"], vec_of_strings!["M", "C", "D"], vec_of_strings!["P"]]; 
    let mut warehouse: Vec<Vec<String>> = vec![
        vec_of_strings!["R", "N", "P", "G"], 
        vec_of_strings!["T", "J", "B", "L", "C", "S", "V", "H"], 
        vec_of_strings!["T", "D", "B", "M", "N", "L"], 
        vec_of_strings!["R", "V", "P", "S", "B"],
        vec_of_strings!["G", "C", "Q", "S", "W", "M", "V", "H"],
        vec_of_strings!["W", "Q", "S", "C", "D", "B", "J"],
        vec_of_strings!["F", "Q", "L"],
        vec_of_strings!["W", "M", "H", "T", "D", "L", "F", "V"],
        vec_of_strings!["L", "P", "B", "V", "M", "J", "F"]
        ];


    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents: Vec<String> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    // day5_1(warehouse, contents);
    day5_2(warehouse, contents);
}

fn day5_1(mut warehouse: Vec<Vec<&str>>, contents: Vec<String>) {
    for direction in contents {
        if direction == "" { break; }
        let move_number: i32 = direction.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        let from_number: i32 = direction.split(" ").collect::<Vec<&str>>()[3].parse().unwrap();
        let to_number: i32 = direction.split(" ").collect::<Vec<&str>>()[5].parse().unwrap();
        for i in 0..move_number {
            let storage_box = warehouse[(from_number - 1) as usize].pop().unwrap();
            warehouse[(to_number-1) as usize].push(storage_box);
        }
    }
    for stack in warehouse {
        print!("{}", stack[stack.len()-1 as usize]);
    }
    println!();
}

fn day5_2(mut warehouse: Vec<Vec<String>>, contents: Vec<String>) {
    // move 1 from 2 to 1
    // move 3 from 1 to 3
    // move 2 from 2 to 1
    // move 1 from 1 to 2
    for direction in contents {
        if direction == "" { break; }
        let move_number: i32 = direction.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        let from_number: i32 = direction.split(" ").collect::<Vec<&str>>()[3].parse().unwrap();
        let to_number: i32 = direction.split(" ").collect::<Vec<&str>>()[5].parse().unwrap();
        let mut tmp: Vec<String> = Vec::new();
        for i in 0..move_number {
            let storage_box = warehouse[(from_number - 1) as usize].pop().unwrap();
            tmp.push(storage_box.to_string());
        }
        let mut tmp2 = tmp.iter().clone().rev();
        for t in tmp2 {
            warehouse[(to_number-1) as usize].push(t.to_string());
        }
    }
    for stack in warehouse {
        print!("{}", stack[stack.len()-1 as usize]);
    }
    println!();
}