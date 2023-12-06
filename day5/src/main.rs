use colored::*;
use std::collections::VecDeque;
use std::env;

/* NOTE: I had to give up and look for hints on P2 because I was ready to flip a table. Thanks to u/zuleyorker for the great writeup! */

fn parse_map(input: String) -> Vec<Vec<i64>> {
    let numbers_str: Vec<&str> = input.split(":\n").collect::<Vec<&str>>()[1].split("\n").collect::<Vec<&str>>();
    let mut numbers_map: Vec<Vec<i64>> = vec![];
    for number_range in numbers_str.into_iter() {
        numbers_map.push(number_range.split(" ").map(|x| x.parse::<i64>().unwrap()).collect());
    }
    return numbers_map;
}

fn translate(source: i64, mapping: Vec<Vec<Vec<i64>>>) -> i64 {
    let mut mapped_val: i64 = source;
    for map in mapping.into_iter() {
        let mut flag: bool = false;
        for line in map.into_iter() {
            if !flag && mapped_val >= line[1] && mapped_val - line[1] < line[2] {
                mapped_val = line[0] + mapped_val - line[1];
                flag = true;
            }
        }
    }
    return mapped_val;
}

fn inverse_translate(source: i64, mapping: Vec<Vec<i64>>) -> i64 {
    for line in mapping.into_iter() {
        if line[0] <= source && source < (line[0] + line[2]) {
            return line[1] + (source - line[0]);
        }
    }
    return source;
}

fn get_map_endpoints(map: Vec<Vec<i64>>, starting_endpoints: Vec<i64>) -> Vec<i64> {
    let mut endpoints: Vec<i64> = vec![];
    for line in map.clone().into_iter() {
        let line_endpoints: Vec<i64> = [line[1], line[1] + line[2] - 1].to_vec();
        if line_endpoints.first().unwrap() > &0 && !endpoints.contains(&(line_endpoints.first().unwrap() - 1)) {
            endpoints.push(line_endpoints.first().unwrap() - 1);
        }
        if line_endpoints.last().unwrap() < &std::i64::MAX && !endpoints.contains(&(line_endpoints.last().unwrap() + 1)) {
            endpoints.push(line_endpoints.last().unwrap() + 1);
        }
        for e in line_endpoints.into_iter() {
            if !endpoints.contains(&e) {
                endpoints.push(e);
            }
        }
    }
    for source in starting_endpoints.into_iter() {
        let inverted = inverse_translate(source, map.clone());
        if !endpoints.contains(&inverted) {
            endpoints.push(inverted);
        }
    }
    endpoints.sort();
    return endpoints;
}

fn get_endpoints(mapping: Vec<Vec<Vec<i64>>>) -> Vec<i64> {
    let mut endpoints: Vec<i64> = [0, std::i64::MAX].to_vec();
    for map in mapping.into_iter().rev() {
        endpoints = get_map_endpoints(map, endpoints.clone());
    }
    return endpoints;
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let mut lines: VecDeque<&str> = input.split("\n\n").collect::<VecDeque<&str>>();
    let seeds_str = lines.pop_front().unwrap().split(": ").collect::<Vec<&str>>()[1];
    let seeds: Vec<i64> = seeds_str.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
    let mut mapping: Vec<Vec<Vec<i64>>> = vec![];
    for line in lines.into_iter() {
        mapping.push(parse_map(line.to_string()));
    }
    let mut locations: Vec<i64> = vec![];
    for seed in seeds.into_iter() {
        locations.push(translate(seed, mapping.clone()));
    }
    let minimum = locations.iter().min();
    println!("ANSWER: {}\n", minimum.unwrap());
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let mut lines: VecDeque<&str> = input.split("\n\n").collect::<VecDeque<&str>>();
    let seeds_str = lines.pop_front().unwrap().split(": ").collect::<Vec<&str>>()[1];
    let seed_ranges: Vec<i64> = seeds_str.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
    let mut mapping: Vec<Vec<Vec<i64>>> = vec![];
    for line in lines.into_iter() {
        mapping.push(parse_map(line.to_string()));
    }
    let endpoints = get_endpoints(mapping.clone());
    let mut intersecting_endpoints: Vec<i64> = vec![];
    for i in 0..(seed_ranges.len() / 2) {
        let range_start = seed_ranges[i * 2];
        let range_size = seed_ranges[i * 2 + 1] - 1;
        for e in endpoints.clone().into_iter() {
            if e > range_start && e < range_start + range_size {
                intersecting_endpoints.push(e);
            }
        }
        intersecting_endpoints.push(range_start);
        intersecting_endpoints.push(range_start + range_size);
    }
    intersecting_endpoints.sort();
    let mut translated_endpoints: Vec<i64> = vec![];
    for e in intersecting_endpoints.into_iter() {
        translated_endpoints.push(translate(e, mapping.clone()));
    }
    let minimum = translated_endpoints.iter().min();
    println!("ANSWER: {}\n", minimum.unwrap());
}

fn help() {
    println!("{}\n", "Please pass in either 1 or 2".yellow().bold())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: usize = 5;
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
