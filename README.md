# Rust setup for Advent of Code

This is a relatively generalized Rust setup for advent of code. The input data for each challenge is fetched and cached locally.

To set up:

1. Clone repo
2. Install rust (see https://www.rust-lang.org/tools/install)
3. Copy the value of the Advent of Code session cookie from your [browser's devtools](https://developer.chrome.com/docs/devtools/storage/cookies/).
4. Add that value with no whitespace to a new file "session_cookie.txt" in the root of the directory. (e.g. `echo -n "$cookie_value" > session_cookie.txt`)
5. Execute `cargo run -- 1` from directory root to execute the challenge for day 1.

To bootstrap a new day:

Run `./bin/add-day.sh $day`, where `$day` is a number. This will bootstrap everything and should compile immediately. You can optionally pass another argument to set the function name, like `./bin/add-day.sh 10 fun_holidays`. You can keep the default naming scheme or use something that describes the challenge at hand :)

To add a new year:

1. Create a new year directory like y2023. (Should follow this same format.)
2. Copy the mod.rs file from an existing year to this new directory.
3. Remove the `mod dayX;` lines, as well as everything in the match statement except the base error case. It should then be mostly empty.
4. Change the year name at the top of `src/main.rs`. (Both AOC_YEAR and the mod line for the year. We don't need to add the module for other years that we won't use.)
5. Run `./bin/add-day.sh 1` to create the first day file -- this will ultimately update the year's mod.rs file. From here you can just add new days above!
