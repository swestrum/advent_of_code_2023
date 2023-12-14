use clap::Parser;
use colored::*;
use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;

/* NOTE: Is it good? Not at all. Does it work? Yep. */

#[derive(Parser, Default, Debug)]
struct Args {
    #[arg(short, long)]
    test: bool,

    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

fn parse_input(input: String, unfold: bool) -> Vec<(Vec<String>, Vec<usize>)> {
    let lines: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut structured_records = vec![];
    for line in lines.into_iter() {
        let arrangement_record = line.split(" ").collect::<Vec<&str>>();
        let mut arrangement_str = arrangement_record[0].replace(".", " ");
        if unfold {
            arrangement_str = arrangement_str + "?";
            arrangement_str = arrangement_str.repeat(5);
            arrangement_str.pop();
        }
        let arrangement = arrangement_str.split_whitespace().map(String::from).collect::<Vec<String>>();
        let mut record: Vec<usize> = arrangement_record[1]
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        if unfold {
            record = [&record[..], &record[..], &record[..], &record[..], &record[..]].concat();
        }
        structured_records.push((arrangement, record));
    }
    return structured_records;
}

fn match_occurrences(arrangement: &str, record: usize) -> bool {
    let regex_str = [r"^[#|\?]{", record.to_string().as_str(), r"}(\?|$)"].join("");
    let re = Regex::new(&regex_str).unwrap();
    return re.find_iter(arrangement).count() > 0;
}

fn count_permutations(
    mut arrangement: Vec<String>,
    mut record: Vec<usize>,
    mut saved_results: HashMap<(Vec<String>, Vec<usize>), usize>,
) -> (usize, HashMap<(Vec<String>, Vec<usize>), usize>) {
    if saved_results.contains_key(&(arrangement.clone(), record.clone())) {
        return (*saved_results.get(&(arrangement.clone(), record.clone())).unwrap(), saved_results);
    }
    if record.len() == 0 {
        for a in arrangement.iter() {
            if !a.replace("?", "").is_empty() {
                return (0, saved_results);
            }
        }
        return (1, saved_results);
    }
    if arrangement.len() == 0 {
        return (0, saved_results);
    }
    let first_arrangement = arrangement[0].clone();
    if first_arrangement.is_empty() {
        arrangement.remove(0);
        return count_permutations(arrangement, record, saved_results);
    }
    if first_arrangement.starts_with("#") {
        if match_occurrences(&first_arrangement, record[0]) {
            let num_remove = min(record[0] + 1, arrangement[0].len());
            arrangement[0].drain(0..num_remove);
            record.remove(0);
            return count_permutations(arrangement, record, saved_results);
        }
        return (0, saved_results);
    }
    arrangement[0].drain(0..1);
    let (empty_size, empty_saved_results) = count_permutations(arrangement.clone(), record.clone(), saved_results.clone());
    saved_results.insert((arrangement.clone(), record.clone()), empty_size);
    saved_results.extend(empty_saved_results);
    arrangement[0] = "#".to_string() + &arrangement[0];
    let (full_size, full_saved_results) = count_permutations(arrangement.clone(), record.clone(), saved_results.clone());
    saved_results.insert((arrangement.clone(), record.clone()), full_size);
    saved_results.extend(full_saved_results);
    return (empty_size + full_size, saved_results);
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let structured_records = parse_input(input, false);
    let mut permutation_sum = 0;
    for (arrangement, record) in structured_records.into_iter() {
        let saved_results = HashMap::new();
        let (permutation_ind, _) = count_permutations(arrangement, record, saved_results);
        permutation_sum += permutation_ind;
    }
    println!("ANSWER: {}\n", permutation_sum);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let structured_records = parse_input(input, true);
    let mut permutation_sum = 0;
    let mut num_calculated = 0;
    for (arrangement, record) in structured_records.into_iter() {
        let saved_results = HashMap::new();
        let (single_permut, _) = count_permutations(arrangement, record, saved_results);
        num_calculated += 1;
        println!("({}) Found {}", num_calculated, single_permut);
        permutation_sum += single_permut;
    }
    println!("ANSWER: {}\n", permutation_sum);
}

fn main() {
    let args = Args::parse();
    let day: usize = 12;
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
