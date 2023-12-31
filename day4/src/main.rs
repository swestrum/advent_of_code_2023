use clap::Parser;
use colored::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Parser, Default, Debug)]
struct Args {
    #[arg(short, long)]
    test: bool,

    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

fn parse_card(card_line: String) -> Vec<Vec<usize>> {
    let numbers: String = card_line.split(": ").collect::<Vec<&str>>()[1].to_string();
    let winning_nums_str: &str = numbers.split(" | ").collect::<Vec<&str>>()[1];
    let card_str: &str = numbers.split(" | ").collect::<Vec<&str>>()[0];
    let mut winning_nums: Vec<usize> = vec![];
    let mut card: Vec<usize> = vec![];
    let re = Regex::new(r"(\d+)").unwrap();
    for cap in re.captures_iter(winning_nums_str) {
        let num = cap.get(0).unwrap().as_str().to_string().parse::<usize>().unwrap();
        winning_nums.push(num);
    }
    for cap in re.captures_iter(card_str) {
        let num = cap.get(0).unwrap().as_str().to_string().parse::<usize>().unwrap();
        card.push(num);
    }
    return [card, winning_nums].to_vec();
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut total: isize = 0;
    for line in lines.into_iter() {
        let parsed_card = parse_card(line.to_string());
        let mut matching_nums: u32 = 0;
        for card_num in parsed_card[0].clone().into_iter() {
            if parsed_card[1].clone().contains(&card_num) {
                matching_nums += 1;
            }
        }
        if matching_nums > 0 {
            total += (2 as isize).pow(matching_nums - 1);
        }
    }
    println!("ANSWER: {}\n", total);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut line_multiples: HashMap<usize, usize> = HashMap::new();
    let mut line_points: HashMap<usize, isize> = HashMap::new();
    let mut total: isize = 0;
    for (i, line) in lines.into_iter().enumerate() {
        let parsed_card = parse_card(line.to_string());
        let mut matching_nums: u32 = 0;
        for card_num in parsed_card[0].clone().into_iter() {
            if parsed_card[1].clone().contains(&card_num) {
                matching_nums += 1;
            }
        }
        if matching_nums > 0 {
            line_points.insert(i, (2 as isize).pow(matching_nums - 1));
            for j in i + 1..i + matching_nums as usize + 1 {
                let mult_val = line_multiples.get(&j).unwrap_or(&0) + 1 + line_multiples.get(&i).unwrap_or(&0);
                line_multiples.insert(j, mult_val);
            }
        } else {
            line_points.insert(i, 0);
        }
        total += *line_multiples.get(&i).unwrap_or(&0) as isize + 1
    }
    println!("ANSWER: {}\n", total);
}

fn main() {
    let args = Args::parse();
    let day: usize = 4;
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
