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
    
    // day4_1(contents);
    day4_2(contents);
}

fn day4_1(contents: Vec<String>) {
    let mut count = 0;
    for group in contents {
        if group == "" { break; }
        println!("{}", group);
        let elf1_range = group.split(",").collect::<Vec<&str>>()[0].split("-").collect::<Vec<&str>>();
        let elf2_range = group.split(",").collect::<Vec<&str>>()[1].split("-").collect::<Vec<&str>>();

        let elf1_start: i32 = elf1_range[0].parse().unwrap();
        let elf1_end: i32 = elf1_range[1].parse().unwrap();
        let elf2_start: i32 = elf2_range[0].parse().unwrap();
        let elf2_end: i32 = elf2_range[1].parse().unwrap();
        let elf2_vec: Vec<i32> = (elf2_start..elf2_end+1).collect();
        let elf1_vec: Vec<i32> = (elf1_start..elf1_end+1).collect();

        let mut onefits2: bool = true;
        let mut twofits1: bool = true;
        for i in elf1_start..elf1_end+1 {
            if !elf2_vec.contains(&i) {
                twofits1 = false;
                break;
            }
        }
        for i in elf2_start..elf2_end+1 {
            if !elf1_vec.contains(&i) {
                onefits2 = false;
                break;
            }
        }
        if onefits2 || twofits1 {
            count += 1;
        }
    }
    println!("{}", count);
}

fn day4_2(contents: Vec<String>) {
    let mut count = 0;
    for group in contents {
        if group == "" { break; }
        let elf1_range = group.split(",").collect::<Vec<&str>>()[0].split("-").collect::<Vec<&str>>();
        let elf2_range = group.split(",").collect::<Vec<&str>>()[1].split("-").collect::<Vec<&str>>();

        let elf1_start: i32 = elf1_range[0].parse().unwrap();
        let elf1_end: i32 = elf1_range[1].parse().unwrap();
        let elf2_start: i32 = elf2_range[0].parse().unwrap();
        let elf2_end: i32 = elf2_range[1].parse().unwrap();
        let elf2_vec: Vec<i32> = (elf2_start..elf2_end+1).collect();
        let elf1_vec: Vec<i32> = (elf1_start..elf1_end+1).collect();

        let mut overlap: bool = false;
        for i in elf1_start..elf1_end+1 {
            if elf2_vec.contains(&i) {
                overlap = true;
                break;
            }
        }
        
        if overlap {
            count += 1;
        }
    }
    println!("{}", count);
}