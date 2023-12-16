use clap::Parser;
use colored::*;

#[derive(Parser, Default, Debug)]
struct Args {
    #[arg(short, long)]
    test: bool,

    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

fn parse_input(input: String) -> Vec<String> {
    let steps: Vec<String> = input.split(",").map(String::from).collect::<Vec<String>>();
    return steps;
}

fn calculate_step_value(step: String) -> usize {
    let mut current_value = 0;
    for c in step.chars() {
        current_value += c as usize;
        current_value *= 17;
        current_value = current_value % 256;
    }
    return current_value;
}

fn calculate_box_value(lens_box: Vec<Vec<(String, usize)>>) -> usize {
    let mut sum = 0;
    for (i, lens) in lens_box.into_iter().enumerate() {
        for (slot, (_, focal_length)) in lens.into_iter().enumerate() {
            let mid_sum = (i + 1) * (slot + 1) * focal_length;
            dbg!(mid_sum);
            sum += mid_sum;
        }
    }
    return sum;
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let steps = parse_input(input);
    let mut hash_sum = 0;
    for step in steps.into_iter() {
        hash_sum += calculate_step_value(step);
    }
    println!("ANSWER: {}\n", hash_sum);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let steps = parse_input(input);
    let mut lenses: Vec<Vec<(String, usize)>> = vec![];
    for _ in 0..256 {
        lenses.push(vec![]);
    }
    for mut step in steps.into_iter() {
        if step.contains("=") {
            let label = step.split("=").collect::<Vec<&str>>()[0].to_string();
            let focal_length = step.split("=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
            let box_idx = calculate_step_value(label.clone());
            let mut change_lens = 257;
            for (i, lens) in lenses[box_idx].iter().enumerate() {
                if lens.0 == label {
                    change_lens = i;
                }
            }
            if change_lens < 256 {
                lenses[box_idx][change_lens] = (label.clone(), focal_length);
            } else {
                lenses[box_idx].push((label, focal_length));
            }
        } else {
            step.pop();
            let label = step;
            let box_idx = calculate_step_value(label.clone());
            let mut remove_lens = 257;
            for (i, lens) in lenses[box_idx].iter().enumerate() {
                if lens.0 == label {
                    remove_lens = i;
                }
            }
            if remove_lens < 256 {
                lenses[box_idx].remove(remove_lens);
            }
        }
    }
    dbg!(calculate_box_value(lenses));
}

fn main() {
    let args = Args::parse();
    let day: usize = 15;
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
