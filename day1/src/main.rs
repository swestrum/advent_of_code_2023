// use std::env;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let day: usize = 1;
    let christmas_emoji: String = "\u{1F384}".to_string();
    println!("\nDAY {} {}\n",  day, christmas_emoji.repeat(day));
}
