use clap::Parser;
use colored::*;
use std::collections::HashMap;

#[derive(Parser, Default, Debug)]
struct Args {
    #[arg(short, long)]
    test: bool,

    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut slots = vec![];
    for line in lines.into_iter() {
        slots.push(line.chars().collect::<Vec<char>>());
    }
    return slots;
}

fn tilt_north(mut platform_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for column in 0..platform_map[0].len() {
        let mut changed = true;
        while changed {
            changed = false;
            for row in 0..platform_map.len() - 1 {
                if platform_map[row + 1][column] == 'O' && platform_map[row][column] == '.' {
                    platform_map[row + 1][column] = '.';
                    platform_map[row][column] = 'O';
                    changed = true;
                }
            }
        }
    }
    return platform_map;
}

fn tilt_south(mut platform_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for column in 0..platform_map[0].len() {
        let mut changed = true;
        while changed {
            changed = false;
            for row in 0..platform_map.len() - 1 {
                if platform_map[row + 1][column] == '.' && platform_map[row][column] == 'O' {
                    platform_map[row + 1][column] = 'O';
                    platform_map[row][column] = '.';
                    changed = true;
                }
            }
        }
    }
    return platform_map;
}

fn tilt_east(mut platform_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for row in 0..platform_map.len() {
        let mut changed = true;
        while changed {
            changed = false;
            for column in 0..platform_map[0].len() - 1 {
                if platform_map[row][column] == 'O' && platform_map[row][column + 1] == '.' {
                    platform_map[row][column] = '.';
                    platform_map[row][column + 1] = 'O';
                    changed = true;
                }
            }
        }
    }
    return platform_map;
}

fn tilt_west(mut platform_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for row in 0..platform_map.len() {
        let mut changed = true;
        while changed {
            changed = false;
            for column in 0..platform_map[0].len() - 1 {
                if platform_map[row][column] == '.' && platform_map[row][column + 1] == 'O' {
                    platform_map[row][column] = 'O';
                    platform_map[row][column + 1] = '.';
                    changed = true;
                }
            }
        }
    }
    return platform_map;
}

fn cycle(mut platform_map: Vec<Vec<char>>, num_cycles: usize) -> Vec<Vec<char>> {
    let mut record: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut first_match: usize = 0;
    let mut cycle_len: usize = 0;
    for i in 0..num_cycles {
        platform_map = tilt_north(platform_map);
        platform_map = tilt_west(platform_map);
        platform_map = tilt_south(platform_map);
        platform_map = tilt_east(platform_map);
        if record.contains_key(&platform_map) && first_match == 0 {
            first_match = i;
            record = HashMap::new();
        }
        if record.contains_key(&platform_map) && first_match != 0 {
            cycle_len = i - first_match;
            break;
        }
        record.insert(platform_map.clone(), i);
    }
    let remaining_cycles = (num_cycles - first_match) % cycle_len;
    for _ in 0..remaining_cycles - 1 {
        platform_map = tilt_north(platform_map);
        platform_map = tilt_west(platform_map);
        platform_map = tilt_south(platform_map);
        platform_map = tilt_east(platform_map);
    }
    return platform_map;
}

fn calculate_load(platform_map: Vec<Vec<char>>) -> usize {
    let mut rows_from_south = platform_map.len();
    let mut sum = 0;
    for i in 0..platform_map.len() {
        let count = platform_map[i].iter().filter(|&n| *n == 'O').count();
        sum += count * rows_from_south;
        rows_from_south -= 1;
    }
    return sum;
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let slots = parse_input(input);
    let tilted_platform = tilt_north(slots);
    let load = calculate_load(tilted_platform);
    println!("ANSWER: {}\n", load);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let slots = parse_input(input);
    let tilted_platform = cycle(slots, 1000000000);
    let load = calculate_load(tilted_platform);
    println!("ANSWER: {}\n", load);
}

fn main() {
    let args = Args::parse();
    let day: usize = 14;
    let christmas_emoji: String = "\u{1F384}".to_string();
    let input_str: String;
    if args.test {
        input_str = include_str!("../data/test_input.txt").to_string();
    } else {
        input_str = include_str!("../data/input.txt").to_string();
    }
    println!("\n{} {} {}\n", "DAY".green(), day.to_string().green(), christmas_emoji.repeat(day));
    match args.part {
        1 => {
            part_one(input_str.clone());
        }
        2 => {
            part_two(input_str.clone());
        }
        _ => {
            part_one(input_str.clone());
            part_two(input_str.clone());
        }
    }
}
