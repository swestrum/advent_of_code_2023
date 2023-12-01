use std::env;
use colored::*;

fn part_one(){
    println!("{}\n", "PART ONE".blue())
}

fn part_two(){
    println!("{}\n", "PART TWO".blue())
}

fn help(){
    println!("{}\n", "Please pass in either 1 or 2".yellow().bold())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: usize = 1;
    let christmas_emoji: String = "\u{1F384}".to_string();
    match args.len() {
        1 => {
            println!("\n{} {} {}\n", "DAY".green(), day.to_string().green(), christmas_emoji.repeat(day));
            part_one();
            part_two();
        }
        2 => {
            println!("\n{} {} {}\n", "DAY".green(), day.to_string().green(), christmas_emoji.repeat(day));
            let num = &args[1];
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            match number {
                1 => {
                    part_one();
                }
                2 => {
                    part_two();
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
