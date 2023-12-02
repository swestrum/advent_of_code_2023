use colored::*;
use std::cmp::max;
use std::env;

fn parse_game(input: &str) -> Vec<Vec<usize>> {
    let game_str: &str = input.split(": ").collect::<Vec<&str>>()[1];
    let rounds: Vec<&str> = game_str.split("; ").collect::<Vec<&str>>();
    let mut game_info: Vec<Vec<usize>> = Vec::new();
    for round_str in rounds.into_iter() {
        let mut round_info: Vec<usize> = vec![0; 3];
        let cube_str: Vec<&str> = round_str.split(", ").collect::<Vec<&str>>();
        for color_str in cube_str.into_iter() {
            let cube: Vec<&str> = color_str.split(" ").collect::<Vec<&str>>();
            match cube[1] {
                "red" => {
                    round_info[0] += cube[0].to_string().parse::<usize>().unwrap();
                }
                "green" => {
                    round_info[1] += cube[0].to_string().parse::<usize>().unwrap();
                }
                "blue" => {
                    round_info[2] += cube[0].to_string().parse::<usize>().unwrap();
                }
                _ => {}
            }
        }
        game_info.push(round_info);
    }
    return game_info;
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut total: usize = 0;
    for (i, line) in lines.into_iter().enumerate() {
        let game_num: usize = i + 1;
        let game_info = parse_game(line);
        let mut flag: bool = true;
        for round in game_info.into_iter() {
            if round[0] > 12 || round[1] > 13 || round[2] > 14 {
                flag = false;
            }
        }
        if flag {
            total += game_num;
        }
    }
    println!("ANSWER: {}\n", total);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut total: usize = 0;
    for line in lines.into_iter() {
        let game_info = parse_game(line);
        let mut min_cubes: Vec<usize> = vec![0; 3];
        for round in game_info.into_iter() {
            for (i, cube_info) in round.into_iter().enumerate() {
                min_cubes[i] = max(min_cubes[i], cube_info);
            }
        }
        total += min_cubes.into_iter().product::<usize>();
    }
    println!("ANSWER: {}\n", total);
}

fn help() {
    println!("{}\n", "Please pass in either 1 or 2".yellow().bold())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: usize = 2;
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
