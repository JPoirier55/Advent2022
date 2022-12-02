use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents: Vec<String> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    
    day1_1(contents);
    day1_2(contents);
}

fn day1_2(contents: Vec<String>) {
    let mut all: Vec<i32> = Vec::new();
    let mut current: i32 = 0;

    for food in contents {
        if food == "" {
            all.push(current);
            current = 0;
        } else {
            let food_int: i32 = food.parse().unwrap();
            current = current + food_int;
        }
    }
    all.push(current);

    all.sort();
    let len = all.len();
    let top3 = &all[len-3..];
    let mut top3total = 0;
    for elf in top3 {
        top3total = top3total + elf
    }
    println!("{}", top3total);
}

fn day1_1(contents: Vec<String>) {
    let mut biggest: i32 = 0;
    let mut current: i32 = 0;
    for food in contents {
        if food == "" {
            if current > biggest {
                biggest = current;
            }
            current = 0;
        } else {
            let food_int: i32 = food.parse().unwrap();
            current = current + food_int;
        }
    }
    if current > biggest {
        biggest = current;
    }
    println!("{}", biggest);
}