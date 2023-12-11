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
    let mut galaxy_map: Vec<Vec<usize>> = vec![];
    for line in lines.into_iter() {
        let mut galaxy_line: Vec<usize> = vec![];
        for j in line.to_string().chars() {
            if j == '#' {
                galaxy_line.push(1);
            } else {
                galaxy_line.push(0);
            }
        }
        galaxy_map.push(galaxy_line.clone());
    }
    return galaxy_map;
}

fn find_space(galaxy_map: Vec<Vec<usize>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_columns: Vec<usize> = vec![];
    let mut empty_rows: Vec<usize> = vec![];
    for x in 0..galaxy_map[0].len() {
        let mut column_sum: usize = 0;
        for y in 0..galaxy_map.len() {
            column_sum = column_sum + galaxy_map[y][x];
        }
        if column_sum == 0 {
            empty_columns.push(x);
        }
    }
    for y in 0..galaxy_map.len() {
        if galaxy_map[y].iter().sum::<usize>() == 0 {
            empty_rows.push(y);
        }
    }
    return (empty_columns, empty_rows);
}

fn find_galaxies(galaxy_map: Vec<Vec<usize>>, empty_x: Vec<usize>, empty_y: Vec<usize>, dialation: usize) -> Vec<(usize, usize)> {
    let mut galaxy_coordinates: Vec<(usize, usize)> = vec![];
    let mut x_dialation: usize;
    let mut y_dialation: usize;
    for y in 0..galaxy_map.len() {
        y_dialation = empty_y.iter().filter(|i| y > **i).count();
        for x in 0..galaxy_map[0].len() {
            if galaxy_map[y][x] == 1 {
                x_dialation = empty_x.iter().filter(|i| x > **i).count();
                galaxy_coordinates.push((x + (x_dialation * dialation), y + (y_dialation * dialation)));
            }
        }
    }
    return galaxy_coordinates;
}

fn find_difference(location_1: (usize, usize), location_2: (usize, usize)) -> isize {
    let x_diff: isize = location_1.0 as isize - location_2.0 as isize;
    let y_diff: isize = location_1.1 as isize - location_2.1 as isize;
    return x_diff.abs() + y_diff.abs();
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let galaxy_map = parse_input(input);
    let (empty_x, empty_y) = find_space(galaxy_map.clone());
    let galaxy_coordinates = find_galaxies(galaxy_map.clone(), empty_x, empty_y, 1);
    let mut distances: Vec<isize> = vec![];
    for i in 0..galaxy_coordinates.len() {
        for j in i..galaxy_coordinates.len() {
            distances.push(find_difference(galaxy_coordinates[i], galaxy_coordinates[j]));
        }
    }
    println!("ANSWER: {}\n", distances.iter().sum::<isize>());
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let galaxy_map = parse_input(input);
    let (empty_x, empty_y) = find_space(galaxy_map.clone());
    let galaxy_coordinates = find_galaxies(galaxy_map.clone(), empty_x, empty_y, 999999);
    let mut distances: Vec<isize> = vec![];
    for i in 0..galaxy_coordinates.len() {
        for j in i..galaxy_coordinates.len() {
            distances.push(find_difference(galaxy_coordinates[i], galaxy_coordinates[j]));
        }
    }
    println!("ANSWER: {}\n", distances.iter().sum::<isize>());
}

fn main() {
    let args = Args::parse();
    let day: usize = 11;
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
