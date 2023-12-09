use clap::Parser;
use colored::*;

#[derive(Parser, Default, Debug)]
struct Args {
    #[arg(short, long)]
    test: bool,

    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

fn parse_input(input: String) -> Vec<Vec<isize>> {
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut number_lines: Vec<Vec<isize>> = vec![];
    for line in lines.iter() {
        let numbers_str: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
        let numbers: Vec<isize> = numbers_str.iter().map(|x| x.parse::<isize>().unwrap()).collect();
        number_lines.push(numbers);
    }
    return number_lines;
}

fn find_differences(numbers: &Vec<isize>) -> Vec<isize> {
    let mut differences: Vec<isize> = vec![];
    for i in 0..numbers.len() - 1 {
        differences.push(numbers[i + 1] - numbers[i]);
    }
    return differences;
}

fn check_all_zeros(numbers: &Vec<isize>) -> bool {
    return *numbers.iter().max().unwrap() == 0 && *numbers.iter().min().unwrap() == 0;
}

fn predict_next_number_last(numbers: &Vec<isize>) -> isize {
    let mut ending_numbers: Vec<isize> = [*numbers.last().unwrap()].to_vec();
    let mut differences: Vec<isize> = find_differences(numbers);
    while !check_all_zeros(&differences) {
        ending_numbers.push(*differences.last().unwrap());
        differences = find_differences(&differences);
    }
    return ending_numbers.iter().sum::<isize>();
}

fn predict_next_number_first(numbers: &Vec<isize>) -> isize {
    let mut first_numbers: Vec<isize> = [*numbers.first().unwrap()].to_vec();
    let mut differences: Vec<isize> = find_differences(numbers);
    while !check_all_zeros(&differences) {
        first_numbers.push(*differences.first().unwrap());
        differences = find_differences(&differences);
    }
    first_numbers.reverse();
    let mut predicted: isize = 0;
    for i in first_numbers.iter() {
        predicted = i - predicted;
    }
    return predicted;
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let numbers: Vec<Vec<isize>> = parse_input(input);
    let mut predicted_numbers: Vec<isize> = vec![];
    for number_line in numbers.iter() {
        predicted_numbers.push(predict_next_number_last(number_line));
    }
    let predicted_sum = predicted_numbers.iter().sum::<isize>();
    println!("ANSWER: {}\n", predicted_sum);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let numbers: Vec<Vec<isize>> = parse_input(input);
    let mut predicted_numbers: Vec<isize> = vec![];
    for number_line in numbers.iter() {
        predicted_numbers.push(predict_next_number_first(number_line));
    }
    let predicted_sum = predicted_numbers.iter().sum::<isize>();
    println!("ANSWER: {}\n", predicted_sum);
}

fn main() {
    let args = Args::parse();
    let day: usize = 9;
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
