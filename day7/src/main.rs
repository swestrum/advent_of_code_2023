use clap::Parser;
use colored::*;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Parser, Default, Debug)]
struct Args {
    #[arg(short, long)]
    test: bool,

    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

#[derive(Debug, Default, Clone, Eq)]
struct Hand {
    card_values: Vec<usize>,
    bid: usize,
    rank: usize,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.rank == other.rank && self.card_values == other.card_values;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rank == other.rank {
            return self.card_values.partial_cmp(&other.card_values);
        }
        return self.rank.partial_cmp(&other.rank);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rank == other.rank {
            return self.card_values.cmp(&other.card_values);
        }
        return self.rank.cmp(&other.rank);
    }
}

fn parse_input(input: String, with_jokers: bool) -> Vec<Hand> {
    let hand_strs: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut hands: Vec<Hand> = vec![];
    for hand_str in hand_strs.into_iter() {
        let hand_split: Vec<&str> = hand_str.split(" ").collect::<Vec<&str>>();
        let hand_chars: Vec<char> = hand_split[0].to_string().chars().collect();
        let mut hand_vals: Vec<usize> = vec![];
        for card in hand_chars.iter() {
            let mut card_val: usize = match card {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => 0,
            };
            if with_jokers && card_val == 11 {
                card_val = 1;
            }
            hand_vals.push(card_val);
        }
        let current_hand: Hand = Hand {
            card_values: hand_vals,
            bid: hand_split[1].parse::<usize>().unwrap(),
            rank: get_rank(hand_chars, with_jokers),
        };
        hands.push(current_hand);
    }
    return hands;
}

fn get_rank(hand: Vec<char>, with_jokers: bool) -> usize {
    let mut char_count: HashMap<char, usize> = HashMap::new();
    let mut max_cards = 0;
    for card in hand.iter() {
        let card_count = char_count.get(card).unwrap_or(&0);
        if card_count + 1 > max_cards {
            max_cards = card_count + 1;
        }
        char_count.insert(*card, *card_count + 1);
    }
    if with_jokers {
        let num_jokers = char_count.remove(&'J').unwrap_or(0);
        max_cards = 0;
        for (_, num_cards) in &char_count {
            if *num_cards > max_cards {
                max_cards = *num_cards;
            }
        }
        max_cards = max_cards + num_jokers;
        if char_count.len() == 0 {
            char_count.insert('J', 5);
        }
    }
    let rank: usize = match (char_count.len(), max_cards) {
        (1, 5) => 7,
        (2, 4) => 6,
        (2, 3) => 5,
        (3, 3) => 4,
        (3, 2) => 3,
        (4, 2) => 2,
        (1, 2) => 1,
        _ => 0,
    };
    return rank;
}

fn calculate_winnings(sorted_hand: Vec<Hand>) -> usize {
    let mut winnings: usize = 0;
    for (i, hand) in sorted_hand.iter().enumerate() {
        winnings = winnings + ((i + 1) * hand.bid);
    }
    return winnings;
}

fn part_one(input: String) {
    println!("{}\n", "PART ONE".blue());
    let mut hands: Vec<Hand> = parse_input(input, false);
    hands.sort();
    let winnings = calculate_winnings(hands);
    println!("ANSWER: {}\n", winnings);
}

fn part_two(input: String) {
    println!("{}\n", "PART TWO".blue());
    let mut hands: Vec<Hand> = parse_input(input, true);
    hands.sort();
    let winnings = calculate_winnings(hands);
    println!("ANSWER: {}\n", winnings);
}

fn main() {
    let args = Args::parse();
    let day: usize = 7;
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
