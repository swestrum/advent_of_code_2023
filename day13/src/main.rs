use clap::Parser;
use colored::*;
use std::cmp::min;

#[derive(Parser, Default, Debug)]
struct Args {
    #[arg(short, long)]
    test: bool,

    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

#[derive(Debug, Default, Clone)]
struct Pattern {
    rows: Vec<String>,
    columns: Vec<String>,
}

fn parse_input(input: String) -> Vec<Pattern> {
    let mirror_strings: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();
    let mut patterns = vec![];
    for mirror in mirror_strings.into_iter() {
        let rows = mirror.split("\n").map(String::from).collect::<Vec<String>>();
        let mut columns = vec![];
        let mut chars = vec![];
        for row in rows.iter() {
            chars.push(row.clone().chars().collect::<Vec<char>>());
        }
        for i in 0..rows[0].len() {
            let mut column = String::from("");
            for j in 0..rows.len() {
                column.push(chars[j][i]);
            }
            columns.push(column);
        }
        let pattern = Pattern {
            rows: rows,
            columns: columns,
        };
        patterns.push(pattern);
    }
    return patterns;
}

fn check_matching(lines: &[String]) -> bool {
    for i in 0..lines.len() / 2 {
        if lines[i] != lines[lines.len() - 1 - i] {
            return false;
        }
    }
    return true;
}

fn check_almost_matching(lines: &[String]) -> bool {
    let mut hamming_sum = 0;
    for i in 0..lines.len() / 2 {
        let line_a = lines[i].clone();
        let line_b = lines[lines.len() - 1 - i].clone();
        if line_a != line_b {
            hamming_sum += line_a.chars().zip(line_b.chars()).filter(|x| x.0 != x.1).count();
            if hamming_sum > 1 {
                return false;
            }
        }
    }
    return hamming_sum == 1;
}

fn calculate_reflections(patterns: Vec<Pattern>) -> usize {
    let mut summarize = 0;
    for pattern in patterns.into_iter() {
        let num_rows = pattern.rows.len();
        let mut found_match = false;
        for row in 1..num_rows {
            let half_rows = min(num_rows - row, row);
            if check_matching(&pattern.rows[row - half_rows..row + half_rows]) {
                summarize += 100 * row;
                found_match = true;
                break;
            }
        }
        if !found_match {
            let num_cols = pattern.columns.len();
            for column in 1..num_cols {
                let half_cols = min(num_cols - column, column);
                if check_matching(&pattern.columns[column - half_cols..column + half_cols]) {
                    summarize += column;
                    break;
                }
            }
        }
    }
    return summarize;
}

fn calculate_smudges(patterns: Vec<Pattern>) -> usize {
    let mut summarize = 0;
    for pattern in patterns.into_iter() {
        let num_rows = pattern.rows.len();
        let mut found_match = false;
        for row in 1..num_rows {
            let half_rows = min(num_rows - row, row);
            if check_almost_matching(&pattern.rows[row - half_rows..row + half_rows]) {
                summarize += 100 * row;
                found_match = true;
                break;
            }
        }
        if !found_match {
            let num_cols = pattern.columns.len();
            for column in 1..num_cols {
                let half_cols = min(num_cols - column, column);
                if check_almost_matching(&pattern.columns[column - half_cols..column + half_cols]) {
                    summarize += column;
                    break;
                }
            }
        }
    }
    return summarize;
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let patterns = parse_input(input);
    let notes_summary = calculate_reflections(patterns);
    println!("ANSWER: {}\n", notes_summary);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let patterns = parse_input(input);
    let notes_summary = calculate_smudges(patterns);
    println!("ANSWER: {}\n", notes_summary);
}

fn main() {
    let args = Args::parse();
    let day: usize = 13;
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
