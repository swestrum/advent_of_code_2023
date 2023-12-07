use clap::Parser;
use colored::*;

#[derive(Parser, Default, Debug)]
struct Args {
    #[arg(short, long)]
    test: bool,

    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

fn parse_input(input: String) -> Vec<Vec<usize>> {
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut parsed_values: Vec<Vec<usize>> = vec![];
    let mut times_str: Vec<&str> = lines[0].split_whitespace().collect();
    times_str.remove(0);
    let times: Vec<usize> = times_str.iter().map(|x| x.parse::<usize>().unwrap()).collect();
    let mut distances_str: Vec<&str> = lines[1].split_whitespace().collect();
    distances_str.remove(0);
    let distances: Vec<usize> = distances_str.iter().map(|x| x.parse::<usize>().unwrap()).collect();
    for i in 0..times.len() {
        parsed_values.push([times[i], distances[i]].to_vec());
    }
    return parsed_values;
}

fn parse_input_p2(input: String) -> Vec<usize> {
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let time: usize = lines[0].replace(" ", "").split(":").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
    let dist: usize = lines[1].replace(" ", "").split(":").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
    return [time, dist].to_vec();
}

fn calculate_distance(button_hold: usize, race_max: usize) -> usize {
    return (race_max - button_hold) * button_hold;
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let time_and_dist: Vec<Vec<usize>> = parse_input(input);
    let mut num_solutions: usize = 1;
    for races in time_and_dist.into_iter() {
        let mut record_distance = 0;
        let mut button_hold = 0;
        while record_distance <= races[1] {
            button_hold += 1;
            record_distance = calculate_distance(button_hold, races[0]);
        }
        let min_button_hold = button_hold;
        record_distance = 0;
        button_hold = races[0];
        while record_distance <= races[1] {
            button_hold -= 1;
            record_distance = calculate_distance(button_hold, races[0]);
        }
        let max_button_hold = button_hold;
        num_solutions = num_solutions * (max_button_hold - min_button_hold + 1);
    }
    println!("ANSWER: {}\n", num_solutions);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let time_and_dist: Vec<usize> = parse_input_p2(input);
    let mut record_distance = 0;
    let mut button_hold = 0;
    while record_distance <= time_and_dist[1] {
        button_hold += 1;
        record_distance = calculate_distance(button_hold, time_and_dist[0]);
    }
    let min_button_hold = button_hold;
    record_distance = 0;
    button_hold = time_and_dist[0];
    while record_distance <= time_and_dist[1] {
        button_hold -= 1;
        record_distance = calculate_distance(button_hold, time_and_dist[0]);
    }
    let max_button_hold = button_hold;
    println!("ANSWER: {}\n", max_button_hold - min_button_hold + 1);
}

fn main() {
    let args = Args::parse();
    let day: usize = 6;
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
