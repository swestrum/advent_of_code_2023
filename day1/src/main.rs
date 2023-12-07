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

fn part_one(input: String, quiet: bool) {
    if !quiet {
        println!("{}\n", "PART ONE".blue());
    }
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut total: usize = 0;
    for line in lines.into_iter() {
        let mut numbers: Vec<usize> = Vec::with_capacity(10);
        for c in line.chars() {
            match c.to_string().parse() {
                Ok(n) => numbers.push(n),
                _ => {}
            }
        }
        let line_total: usize = (numbers.first().unwrap() * 10 as usize) + numbers.last().unwrap();
        total += line_total;
    }
    println!("ANSWER: {}\n", total);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let mut text_numbers = HashMap::new();
    text_numbers.insert("one", "one1one");
    text_numbers.insert("two", "two2two");
    text_numbers.insert("three", "three3three");
    text_numbers.insert("four", "four4four");
    text_numbers.insert("five", "five5five");
    text_numbers.insert("six", "six6six");
    text_numbers.insert("seven", "seven7seven");
    text_numbers.insert("eight", "eight8eight");
    text_numbers.insert("nine", "nine9nine");
    let mut input_cleaned = input.clone();
    for (key, value) in text_numbers {
        input_cleaned = input_cleaned.replace(key, value)
    }
    part_one(input_cleaned.clone(), true);
}

fn main() {
    let args = Args::parse();
    let day: usize = 1;
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
            part_one(input_str.clone(), false);
        }
        2 => {
            part_two(input_str.clone());
        }
        _ => {
            part_one(input_str.clone(), false);
            part_two(input_str.clone());
        }
    }
}
