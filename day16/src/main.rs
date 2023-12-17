use clap::Parser;
use colored::*;
use std::collections::{HashMap, HashSet};

#[derive(Parser, Default, Debug)]
struct Args {
    #[arg(short, long)]
    test: bool,

    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

fn parse_input(input: String) -> (HashMap<(usize, usize), char>, usize) {
    let rows: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    for (y, row) in rows.clone().into_iter().enumerate() {
        let row_chars = row.chars().collect::<Vec<char>>();
        for (x, node_char) in row_chars.into_iter().enumerate() {
            map.insert((x + 1, y + 1), node_char);
        }
    }
    return (map, rows.len());
}

fn translate_momentum(location: char, momentum: (isize, isize)) -> Vec<(isize, isize)> {
    assert!(momentum.0 != 0 || momentum.1 != 0);
    let mut new_momentum = vec![];
    if location == '.' {
        new_momentum.push(momentum);
        return new_momentum;
    }
    if location == '|' {
        if momentum.1 != 0 {
            new_momentum.push(momentum);
            return new_momentum;
        }
        new_momentum.push((0, 1));
        new_momentum.push((0, -1));
        return new_momentum;
    }
    if location == '-' {
        if momentum.0 != 0 {
            new_momentum.push(momentum);
            return new_momentum;
        }
        new_momentum.push((1, 0));
        new_momentum.push((-1, 0));
        return new_momentum;
    }
    if location == '\\' {
        if momentum == (1, 0) {
            new_momentum.push((0, 1));
            return new_momentum;
        }
        if momentum == (0, -1) {
            new_momentum.push((-1, 0));
            return new_momentum;
        }
        if momentum == (-1, 0) {
            new_momentum.push((0, -1));
            return new_momentum;
        }
        if momentum == (0, 1) {
            new_momentum.push((1, 0));
            return new_momentum;
        }
    }
    if location == '/' {
        if momentum == (1, 0) {
            new_momentum.push((0, -1));
            return new_momentum;
        }
        if momentum == (0, -1) {
            new_momentum.push((1, 0));
            return new_momentum;
        }
        if momentum == (-1, 0) {
            new_momentum.push((0, 1));
            return new_momentum;
        }
        if momentum == (0, 1) {
            new_momentum.push((-1, 0));
            return new_momentum;
        }
    }
    unreachable!();
}

fn trace_light(
    map: &HashMap<(usize, usize), char>,
    coordinates: (usize, usize),
    momentum: (isize, isize),
    max: usize,
    mut visited: HashSet<((usize, usize), (isize, isize))>,
) -> HashSet<((usize, usize), (isize, isize))> {
    if coordinates.0 > max || coordinates.1 > max || coordinates.0 == 0 || coordinates.1 == 0 || visited.contains(&(coordinates, momentum)) {
        return visited;
    }
    visited.insert((coordinates, momentum));
    let location_char = map.get(&coordinates).unwrap();
    let new_momentum = translate_momentum(*location_char, momentum);
    for m in new_momentum.into_iter() {
        let new_coordinates = ((coordinates.0 as isize + m.0) as usize, (coordinates.1 as isize + m.1) as usize);
        visited.extend(trace_light(map, new_coordinates, m, max, visited.clone()));
    }
    return visited;
}

fn get_num_visited(visited: HashSet<((usize, usize), (isize, isize))>) -> usize {
    let mut visited_nodes = HashSet::new();
    for (node, _) in visited.into_iter() {
        visited_nodes.insert(node);
    }
    dbg!(visited_nodes.len());
    return visited_nodes.len();
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let (map, max) = parse_input(input);
    let visited = trace_light(&map, (1, 1), (1, 0), max, HashSet::new());
    println!("ANSWER: {}\n", get_num_visited(visited));
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let mut max_tiles = 0;
    let mut num_visited: usize;
    let mut visited: HashSet<((usize, usize), (isize, isize))>;
    let (map, max) = parse_input(input);
    for i in 1..max + 1 {
        println!("Checking {}", i);
        // visited = trace_light(&map, (1, i), (1, 0), max, HashSet::new());
        // num_visited = get_num_visited(visited);
        // if num_visited > max_tiles {
        //     max_tiles = num_visited;
        // }
        // println!("Checked right");
        visited = trace_light(&map, (max, i), (-1, 0), max, HashSet::new());
        num_visited = get_num_visited(visited);
        if num_visited > max_tiles {
            max_tiles = num_visited;
        }
        // println!("Checked left");
        // visited = trace_light(&map, (i, 1), (0, 1), max, HashSet::new());
        // num_visited = get_num_visited(visited);
        // if num_visited > max_tiles {
        //     max_tiles = num_visited;
        // }
        // println!("Checked down");
        visited = trace_light(&map, (i, max), (0, -1), max, HashSet::new());
        num_visited = get_num_visited(visited);
        if num_visited > max_tiles {
            max_tiles = num_visited;
        }
        // println!("Checked up");
    }
    println!("ANSWER: {}\n", max_tiles);
}

fn main() {
    let args = Args::parse();
    let day: usize = 16;
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
