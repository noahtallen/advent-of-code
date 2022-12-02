# Rust setup for Advent of Code

This is a relatively generalized Rust setup for advent of code. The input data for each challenge is fetched and cached locally.

To set up:

1. Clone repo
2. Install rust (see https://www.rust-lang.org/tools/install)
3. Copy the value of the Advent of Code session cookie from your [browser's devtools](https://developer.chrome.com/docs/devtools/storage/cookies/).
4. Add that value with no whitespace to a new file "session_cookie.txt" in the root of the directory. (e.g. `echo -n "$cookie_value" > session_cookie.txt`)
5. Execute `cargo run -- 1` from directory root to execute the challenge for day 1.

To add a new day:

1. Create a new file "dayX.rs" in the year directory with a function accepting a string reference and returning a string.
2. Add `pub mod dayX.rs;` to the `mod.rs` file in the current year directory.
3. In the same `mod.rs` file, add a case to the `match` statement in the `run_day` function for the new day which calls the new function. Like `2 => day2::day_two(input),`
4. Since days use the same input, you can output the answers to both parts from the individual day function.