use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::env;

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut chars: Vec<Vec<char>> = Vec::new();
    for line in lines.clone().into_iter() {
        chars.push(line.chars().collect());
    }
    let mut total: u64 = 0;
    for (i, line) in lines.clone().into_iter().enumerate() {
        let re = Regex::new(r"(\d+)").unwrap();
        for cap in re.captures_iter(line) {
            let num = cap.get(0).unwrap();
            let mut flag: bool = true;
            for row in (i as i32) - 1..(i as i32) + 2 {
                for col in (num.start() as i32) - 1..(num.end() + 1).try_into().unwrap() {
                    let empty_line = &Vec::new();
                    let empty_char = &'.';
                    let check_char = chars.get(row as usize).unwrap_or(empty_line).get(col as usize).unwrap_or(empty_char);
                    match check_char {
                        '.' => {}
                        '0'..='9' => {}
                        _ => {
                            flag = false;
                        }
                    }
                }
            }
            if !flag {
                total += num.as_str().to_string().parse::<u64>().unwrap();
            }
        }
    }
    println!("ANSWER: {}\n", total);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut chars: Vec<Vec<char>> = Vec::new();
    for line in lines.clone().into_iter() {
        chars.push(line.chars().collect());
    }
    let mut total: usize = 0;
    let mut gears: HashMap<Vec<i32>, Vec<usize>> = HashMap::new();
    for (i, line) in lines.clone().into_iter().enumerate() {
        let re = Regex::new(r"(\d+)").unwrap();
        for cap in re.captures_iter(line) {
            let num = cap.get(0).unwrap();
            let parsed_num = num.as_str().to_string().parse::<usize>().unwrap();
            for row in (i as i32) - 1..(i as i32) + 2 {
                for col in (num.start() as i32) - 1..(num.end() + 1).try_into().unwrap() {
                    let empty_line = &Vec::new();
                    let empty_char = &'.';
                    let check_char = chars.get(row as usize).unwrap_or(empty_line).get(col as usize).unwrap_or(empty_char);
                    match check_char {
                        '.' => {}
                        '0'..='9' => {}
                        '*' => {
                            let mut gear_vec = gears.get(&[row, col].to_vec()).cloned().unwrap_or(vec![]);
                            gear_vec.push(parsed_num);
                            gears.insert([row, col].to_vec(), gear_vec);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    for (_, value) in gears {
        if value.len() == 2 {
            total += value[0] * value[1];
        }
    }
    println!("ANSWER: {}\n", total);
}

fn help() {
    println!("{}\n", "Please pass in either 1 or 2".yellow().bold())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: usize = 3;
    let christmas_emoji: String = "\u{1F384}".to_string();
    let input_str: String = include_str!("../data/input.txt").to_string();
    match args.len() {
        1 => {
            println!("\n{} {} {}\n", "DAY".green(), day.to_string().green(), christmas_emoji.repeat(day));
            part_one(input_str.clone());
            part_two(input_str.clone());
        }
        2 => {
            println!("\n{} {} {}\n", "DAY".green(), day.to_string().green(), christmas_emoji.repeat(day));
            let num = &args[1];
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                }
            };
            match number {
                1 => {
                    part_one(input_str.clone());
                }
                2 => {
                    part_two(input_str.clone());
                }
                _ => {
                    eprintln!("error: second argument not a valid part");
                    help();
                }
            }
        }
        _ => {
            help();
        }
    }
}
