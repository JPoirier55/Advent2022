use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents: Vec<String> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    
    // day3_1(contents);
    day3_2(contents);
}

fn day3_1(contents: Vec<String>) {
    let mut total = 0;
    for line in contents {
        if line == "" { break }
        let ruck1 = &line[..line.len()/2];
        let ruck2 = &line[line.len()/2..];
        for item in ruck1.chars() {
            if ruck2.contains(item) {
                total = total + get_item_value(item);
                break
            }
        }
    }
    println!("{}", total);
}

fn day3_2(contents: Vec<String>) {
    let mut total = 0;
    let mut line_count = 0;
    let mut item_maps: Vec<HashMap<String, i32>> = vec![HashMap::new(), HashMap::new(), HashMap::new()];

    for line in contents {
        if line == "" { break }
        for item in line.chars() {
            &item_maps[line_count].insert(item.to_string(), 1);
        }
        
        if line_count == 2 {
            for item in &item_maps[0] {
                if item_maps[1].contains_key(item.0) && item_maps[2].contains_key(item.0) {
                    total = total + get_item_value(item.0.chars().take(1).last().unwrap());
                }
            }
            for i in 0..3 {
                &item_maps[i].clear();
            }
            line_count = 0;
        } else {
            line_count += 1;
        }
    }
    println!("{}", total);
}

fn get_item_value(item: char) -> i32 {
    if item.is_uppercase() {
        return item as i32 - 38;
    } else {
        return item as i32 - 96;
    }
}