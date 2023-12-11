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

fn parse_input(input: String) -> (Vec<Vec<char>>, (usize, usize)) {
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut trail_map: Vec<Vec<char>> = vec![];
    let mut starting_y: usize = 0;
    let mut starting_x: usize = 0;
    for (i, line) in lines.into_iter().enumerate() {
        if line.contains("S") {
            starting_y = i;
            starting_x = line.find("S").unwrap();
        }
        trail_map.push(line.to_string().chars().collect());
    }
    return (trail_map, (starting_x, starting_y));
}

fn build_graph(map: Vec<Vec<char>>, s_x: usize, s_y: usize) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    for (y, line) in map.into_iter().enumerate() {
        for (x, map_char) in line.iter().enumerate() {
            match map_char {
                'L' => {
                    graph.entry((x, y)).or_insert(vec![]).push((x + 1, y));
                    graph.entry((x + 1, y)).or_insert(vec![]).push((x, y));
                    if y > 0 {
                        graph.entry((x, y)).or_insert(vec![]).push((x, y - 1));
                        graph.entry((x, y - 1)).or_insert(vec![]).push((x, y));
                    }
                }
                'J' => {
                    if x > 0 {
                        graph.entry((x, y)).or_insert(vec![]).push((x - 1, y));
                        graph.entry((x - 1, y)).or_insert(vec![]).push((x, y));
                    }
                    if y > 0 {
                        graph.entry((x, y)).or_insert(vec![]).push((x, y - 1));
                        graph.entry((x, y - 1)).or_insert(vec![]).push((x, y));
                    }
                }
                '|' => {
                    graph.entry((x, y)).or_insert(vec![]).push((x, y + 1));
                    graph.entry((x, y + 1)).or_insert(vec![]).push((x, y));
                    if y > 0 {
                        graph.entry((x, y)).or_insert(vec![]).push((x, y - 1));
                        graph.entry((x, y - 1)).or_insert(vec![]).push((x, y));
                    }
                }
                '-' => {
                    graph.entry((x, y)).or_insert(vec![]).push((x + 1, y));
                    graph.entry((x + 1, y)).or_insert(vec![]).push((x, y));
                    if x > 0 {
                        graph.entry((x, y)).or_insert(vec![]).push((x - 1, y));
                        graph.entry((x - 1, y)).or_insert(vec![]).push((x, y));
                    }
                }
                'F' => {
                    graph.entry((x, y)).or_insert(vec![]).push((x + 1, y));
                    graph.entry((x + 1, y)).or_insert(vec![]).push((x, y));
                    graph.entry((x, y)).or_insert(vec![]).push((x, y + 1));
                    graph.entry((x, y + 1)).or_insert(vec![]).push((x, y));
                }
                '7' => {
                    if x > 0 {
                        graph.entry((x, y)).or_insert(vec![]).push((x - 1, y));
                        graph.entry((x - 1, y)).or_insert(vec![]).push((x, y));
                    }
                    graph.entry((x, y)).or_insert(vec![]).push((x, y + 1));
                    graph.entry((x, y + 1)).or_insert(vec![]).push((x, y));
                }
                _ => (),
            }
        }
    }
    let starting_node_edges: Vec<(usize, usize)> = graph.get(&(s_x, s_y)).unwrap().to_vec();
    for (edge_x, edge_y) in starting_node_edges.iter() {
        graph.entry((*edge_x, *edge_y)).or_insert(vec![]).push((s_x, s_y));
        graph.entry((s_x, s_y)).or_insert(vec![]).push((*edge_x, *edge_y));
    }
    return graph;
}

fn traverse_loop(graph: HashMap<(usize, usize), Vec<(usize, usize)>>, start_x: usize, start_y: usize) -> (usize, Vec<(usize, usize)>) {
    let mut x: usize = start_x.clone();
    let mut y: usize = start_y.clone();
    let mut visited: Vec<(usize, usize)> = vec![];
    let mut steps: usize = 0;
    let mut last_steps: usize = 1;
    while last_steps != steps {
        visited.push((x, y));
        last_steps = steps;
        for (x_new, y_new) in graph.get(&(x, y)).unwrap().into_iter() {
            let num_occurrences: usize = graph
                .get(&(x, y))
                .unwrap()
                .iter()
                .filter(|(x_tmp, y_tmp)| *x_tmp == *x_new && *y_tmp == *y_new)
                .count();
            if !visited.contains(&(*x_new, *y_new)) && num_occurrences >= 2 {
                x = *x_new;
                y = *y_new;
                steps = steps + 1;
                break;
            }
        }
    }
    return (steps, visited);
}

fn fill_adjacent(mut trail_map: Vec<Vec<usize>>, trail_map_char: &Vec<Vec<char>>, start_x: usize, start_y: usize) -> Vec<Vec<usize>> {
    if trail_map[start_y][start_x] == 0 {
        trail_map[start_y][start_x] = 1;
        if start_x != 0 {
            trail_map = fill_adjacent(trail_map, trail_map_char, start_x - 1, start_y);
        }
        if start_x + 1 < trail_map[0].len() {
            trail_map = fill_adjacent(trail_map, trail_map_char, start_x + 1, start_y);
        }
        if start_y != 0 {
            trail_map = fill_adjacent(trail_map, trail_map_char, start_x, start_y - 1);
        }
        if start_y + 1 < trail_map.len() {
            trail_map = fill_adjacent(trail_map, trail_map_char, start_x, start_y + 1);
        }
    }
    return trail_map;
}

fn explode_graph(trail_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut trail_map_exploded: Vec<Vec<char>> = vec![];
    let horizontal_chars: &str = "-FL";
    let vertical_chars: &str = "|F7";
    let mut start_x: usize = 0;
    let mut start_y: usize = 0;
    for y in 0..trail_map.len() {
        let mut line: Vec<char> = vec![];
        let mut add_line: Vec<char> = vec!['.'; trail_map[0].len() * 2];
        for x in 0..trail_map[0].len() {
            line.push(trail_map[y][x]);
            if horizontal_chars.contains(trail_map[y][x]) {
                line.push('-');
            } else {
                line.push('.');
            }
            if vertical_chars.contains(trail_map[y][x]) {
                add_line[line.len() - 2] = '|';
            }
            if trail_map[y][x] == 'S' {
                start_x = x * 2;
                start_y = y * 2;
            }
        }
        trail_map_exploded.push(line);
        trail_map_exploded.push(add_line);
    }
    if vertical_chars.contains(trail_map_exploded[start_y - 2][start_x]) {
        trail_map_exploded[start_y - 1][start_x] = '|';
    }
    if vertical_chars.contains(trail_map_exploded[start_y + 2][start_x]) {
        trail_map_exploded[start_y + 1][start_x] = '|';
    }
    if horizontal_chars.contains(trail_map_exploded[start_y][start_x - 2]) {
        trail_map_exploded[start_y][start_x - 1] = '-';
    }
    if horizontal_chars.contains(trail_map_exploded[start_y][start_x + 2]) {
        trail_map_exploded[start_y][start_x + 1] = '-';
    }
    return trail_map_exploded;
}

fn deflate_graph(trail_map: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut deflated_trail_map: Vec<Vec<usize>> = vec![];
    for y in 0..trail_map.len() {
        if y % 2 == 0 {
            let mut line: Vec<usize> = vec![];
            for x in 0..trail_map[0].len() {
                if x % 2 == 0 {
                    line.push(trail_map[y][x]);
                }
            }
            deflated_trail_map.push(line);
        }
    }
    return deflated_trail_map;
}

fn find_loops(trail_map_original: Vec<Vec<char>>, visited: Vec<(usize, usize)>, mut dim_x: usize, mut dim_y: usize) -> usize {
    let mut trail_map_char: Vec<Vec<char>> = vec![vec!['.'; dim_x]; dim_y];
    for (node_x, node_y) in visited.into_iter() {
        trail_map_char[node_y][node_x] = trail_map_original[node_y][node_x];
    }
    let trail_map_exploded: Vec<Vec<char>> = explode_graph(trail_map_char.clone());
    dim_x = trail_map_exploded[0].len();
    dim_y = trail_map_exploded.len();
    let mut trail_map: Vec<Vec<usize>> = vec![vec![0; dim_x]; dim_y];
    for y in 0..dim_y {
        for x in 0..dim_x {
            if trail_map_exploded[y][x] != '.' {
                trail_map[y][x] = 1;
            }
        }
    }
    println!("Examining graph...");
    for x in 0..dim_x {
        trail_map = fill_adjacent(trail_map, &trail_map_exploded, x, 0);
        trail_map = fill_adjacent(trail_map, &trail_map_exploded, x, dim_y - 1);
    }
    for y in 0..dim_y {
        trail_map = fill_adjacent(trail_map, &trail_map_exploded, 0, y);
        trail_map = fill_adjacent(trail_map, &trail_map_exploded, dim_x - 1, y);
    }
    println!("Deflating graph...");
    trail_map = deflate_graph(trail_map);
    let num_inner: usize = ((dim_x / 2) * (dim_y / 2)) - trail_map.iter().flatten().sum::<usize>();
    return num_inner;
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let (trail_map, (x, y)) = parse_input(input);
    let graph: HashMap<(usize, usize), Vec<(usize, usize)>> = build_graph(trail_map, x, y);
    let (steps, _) = traverse_loop(graph, x, y);
    println!("ANSWER: {}\n", steps / 2);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let (trail_map, (x, y)) = parse_input(input);
    let dim_x: usize = trail_map[0].len();
    let dim_y: usize = trail_map.len();
    let graph: HashMap<(usize, usize), Vec<(usize, usize)>> = build_graph(trail_map.clone(), x, y);
    let (_, visited) = traverse_loop(graph, x, y);
    dbg!(find_loops(trail_map.clone(), visited, dim_x, dim_y));
}

fn main() {
    let args = Args::parse();
    let day: usize = 10;
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
