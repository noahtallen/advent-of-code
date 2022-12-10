use std::process;

// Note: avoiding the newline before run_day allows us to easily insert new days
// with a bash script.
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
pub fn run_day(day: u16, input: &String, part_two: bool) -> String {
    match day {
        1 => day1::elves_by_food(input, part_two),
        2 => day2::get_score_from_elf_rps_game(input, part_two),
        3 => day3::rucksack_calculations(input, part_two),
        4 => day4::assignment_checker(input, part_two),
        5 => day5::crate_rearrangement(input, part_two),
        6 => day6::fix_comms_device(input, part_two),
        7 => day7::directory_sizes(input, part_two),
        8 => day8::check_tree_visibility(input, part_two),
        9 => day9::rope_shenanigans(input, part_two),
        10 => day10::signal_strength(input, part_two),
        _ => {
            println!("Day {} not implemented yet", day);
            process::exit(1)
        }
    }
}
