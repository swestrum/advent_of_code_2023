use colored::*;
use std::collections::HashMap;
use std::env;

fn part_one(input: String, quiet: bool) {
    if quiet {
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
    println!("ANSWER: {}", total);
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
    // dbg!(input_cleaned);
    part_one(input_cleaned.clone(), false);
}

fn help() {
    println!("{}\n", "Please pass in either 1 or 2".yellow().bold())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: usize = 1;
    let christmas_emoji: String = "\u{1F384}".to_string();
    let input_str: String = include_str!("../data/input.txt").to_string();
    match args.len() {
        1 => {
            println!("\n{} {} {}\n", "DAY".green(), day.to_string().green(), christmas_emoji.repeat(day));
            part_one(input_str.clone(), true);
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
                    part_one(input_str.clone(), true);
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
