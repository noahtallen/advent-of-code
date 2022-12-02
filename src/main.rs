use std::env;
use std::process;
mod helpers;
// Change next three lines for the new year :)
pub static AOC_YEAR: u16 = 2022;
mod y2022;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please pass an argument for the day.");
        process::exit(1);
    }
    let day = args[1].parse::<u16>().expect("Could not parse day number");
    println!("Running challenge for day: {}", day);

    let input = helpers::get_aoc_input(day).await;

    if has_arg("--show-input") {
        println!("Input: {}", input);
    }

    println!("Running exercise...\n");
    let result = y2022::run_day(day, &input);

    if result.is_empty() {
        println!("Result is empty!");
        process::exit(1);
    }

    println!("\nResult: {}", result);
}

fn has_arg(arg: &str) -> bool {
    let args: Vec<String> = env::args().collect();
    args.contains(&arg.to_string())
}
