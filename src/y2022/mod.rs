use std::process;

// Note: avoiding the newline before run_day allows us to easily insert new days
// with a bash script.
mod day1;
mod day2;
mod day3;
pub fn run_day(day: u16, input: &String, part_two: bool) -> String {
    match day {
        1 => day1::elves_by_food(input, part_two),
        2 => day2::get_score_from_elf_rps_game(input, part_two),
        3 => day3::rucksack_calculations(input, part_two),
        _ => {
            println!("Day {} not implemented yet", day);
            process::exit(1)
        }
    }
}
