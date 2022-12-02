use std::process;
pub mod day1;

pub fn run_day(day: u16, input: &String) -> String {
    match day {
        1 => day1::elves_by_food(input),
        _ => {
            println!("Day {} not implemented yet", day);
            process::exit(1)
        }
    }
}
