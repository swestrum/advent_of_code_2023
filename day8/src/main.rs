use clap::Parser;
use colored::*;
use regex::Regex;
use std::collections::HashMap;

/*
NOTE: Knew that I needed to find cycles in the graph and find the LCM of the cycle lengths to get an answer.
      After two attempts at implementing myself without getting it fast enough, looked up Rust solutions.
      Thanks u/Economics-Repulsive for the great write-up and explanation!
*/

#[derive(Parser, Default, Debug)]
struct Args {
    #[arg(short, long)]
    test: bool,

    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

#[derive(Debug, Default, Clone)]
struct Node {
    left: String,
    right: String,
}

fn parse_input(input: String) -> (Vec<char>, HashMap<String, Node>) {
    let directions_nodes: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();
    let directions: Vec<char> = directions_nodes[0].to_string().chars().collect();
    let nodes: Vec<&str> = directions_nodes[1].split("\n").collect::<Vec<&str>>();
    let re = Regex::new(r"([A-Z\d]{3}) = \(([A-Z\d]{3}), ([A-Z\d]{3})\)").unwrap();
    let mut nodes_map: HashMap<String, Node> = HashMap::new();
    for node in nodes.iter() {
        for cap in re.captures_iter(node) {
            nodes_map.insert(
                cap.get(1).unwrap().as_str().to_string(),
                Node {
                    left: cap.get(2).unwrap().as_str().to_string(),
                    right: cap.get(3).unwrap().as_str().to_string(),
                },
            );
        }
    }
    return (directions, nodes_map);
}

fn steps_to_end(start_node: String, directions: Vec<char>, nodes_map: &HashMap<String, Node>) -> usize {
    let mut current_node = start_node.clone();
    let mut steps = 0;
    let directions_length = directions.len();
    while !current_node.ends_with("Z") {
        current_node = match directions[steps % directions_length] {
            'L' => nodes_map.get(&current_node).unwrap().left.clone(),
            'R' => nodes_map.get(&current_node).unwrap().right.clone(),
            _ => unreachable!(),
        };
        steps = steps + 1;
    }
    return steps;
}

fn greatest_common_denominator(mut n1: usize, mut n2: usize) -> usize {
    while n1 != 0 {
        if n1 < n2 {
            std::mem::swap(&mut n1, &mut n2);
        }
        n1 %= n2;
    }
    return n2;
}

fn least_common_multiple(n1: usize, n2: usize) -> usize {
    return n1.clone() * n2.clone() / greatest_common_denominator(n1, n2);
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let (directions, nodes_map) = parse_input(input);
    let steps = steps_to_end("AAA".to_string(), directions, &nodes_map);
    println!("ANSWER: {}\n", steps);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let (directions, nodes_map) = parse_input(input);
    let mut starting_points: Vec<&str> = vec![];
    for node in nodes_map.keys() {
        if node.ends_with('A') {
            starting_points.push(node);
        }
    }
    let mut length_to_end: Vec<usize> = vec![];
    for start in starting_points.iter() {
        length_to_end.push(steps_to_end(start.to_string(), directions.clone(), &nodes_map));
    }
    let steps = length_to_end.into_iter().fold(1, least_common_multiple);
    println!("ANSWER: {}\n", steps);
}

fn main() {
    let args = Args::parse();
    let day: usize = 8;
    let christmas_emoji: String = "\u{1F384}".to_string();
    let input_str: String;
    let input_str_two: String;
    if args.test {
        input_str = include_str!("../data/test_input.txt").to_string();
        input_str_two = include_str!("../data/test_input_2.txt").to_string();
    } else {
        input_str = include_str!("../data/input.txt").to_string();
        input_str_two = include_str!("../data/input.txt").to_string();
    }
    println!("\n{} {} {}\n", "DAY".green(), day.to_string().green(), christmas_emoji.repeat(day));
    match args.part {
        1 => {
            part_one(input_str.clone());
        }
        2 => {
            part_two(input_str_two.clone());
        }
        _ => {
            part_one(input_str.clone());
            part_two(input_str_two.clone());
        }
    }
}
