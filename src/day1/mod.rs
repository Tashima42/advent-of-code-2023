use std::{env, fs::read_to_string};

pub fn run() {
    let current_dir = env::current_dir().unwrap();
    let filename = current_dir.join("src/day1/input.txt");

    let mut total: u32 = 0;

    for line in read_to_string(filename).unwrap().lines() {
        let chars = line.chars();
        let mut first_num = "".to_string();
        let mut last_num = "".to_string();
        for c in chars {
            if c.is_digit(10) {
                if first_num == "" {
                    first_num = c.to_string();
                    last_num = c.to_string();
                } else {
                    last_num = c.to_string();
                }
            }
        }
        first_num.push_str(&last_num);
        let joined_nums: u32 = first_num.parse().unwrap();
        total += joined_nums;
    }
    println!("Total: {}", total);
}
