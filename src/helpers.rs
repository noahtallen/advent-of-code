use crate::AOC_YEAR;
use reqwest::header::COOKIE;
use std::fs;
use std::process;

pub async fn get_aoc_input(day: u16) -> String {
    if let Ok(input) = read_aoc_input_file(day) {
        if !input.is_empty() {
            println!("Found local input data!");
            return input;
        }
    }
    println!("Fetching data from remote instead...");

    let result = get_aoc_input_from_remote(day).await;

    if !result.is_empty() {
        println!("Writing input data...");
        write_aoc_input_file(day, &result);
        return result;
    } else {
        println!("Input data from remote is empty!");
        process::exit(1);
    }
}

async fn get_aoc_input_from_remote(day: u16) -> String {
    let session_cookie = read_file("session_cookie.txt", "Could not read session cookie");

    let client = reqwest::Client::new();
    let advent_url = format!("https://adventofcode.com/{}/day/{}/input", AOC_YEAR, day);
    let result = client
        .get(advent_url)
        .header(COOKIE, format!("session={}", session_cookie))
        .send()
        .await
        .expect("Remote fetch failed");

    let status = result.status();
    if status.is_success() {
        return result.text().await.expect("Could not parse response text.");
    } else {
        println!("Data fetch unsuccessful! Code: {}.", status.as_u16());
        println!("Double check you are authenticated.");
        process::exit(1);
    }
}

fn get_aoc_input_filename(day: u16) -> String {
    format!("input/{}day{:02}.txt", AOC_YEAR, day)
}

fn write_aoc_input_file(day: u16, contents: &String) {
    fs::create_dir_all("input/").expect("Could not create input directory");
    fs::write(get_aoc_input_filename(day), &contents).expect("Could not write input file");
}

fn read_aoc_input_file(day: u16) -> Result<String, std::io::Error> {
    fs::read_to_string(get_aoc_input_filename(day))
}

fn read_file(path: &str, err_msg: &str) -> String {
    match fs::read_to_string(path) {
        Ok(file_contents) => {
            if file_contents.trim().is_empty() {
                println!("{}: File is empty.", err_msg);
                process::exit(1);
            }
            return file_contents;
        }
        Err(e) => {
            eprintln!("{}: {}", err_msg, e);
            process::exit(1);
        }
    }
}
