use std::process;

// Note: avoiding the newline before run_day allows us to easily insert new days
// with a bash script.
mod day1;
mod day2;
pub fn run_day(day: u16, input: &String) -> String {
    match day {
        1 => day1::elves_by_food(input),
        2 => day2::day_two(input),
        _ => {
            println!("Day {} not implemented yet", day);
            process::exit(1)
        }
    }
}
