#!/bin/bash
set -Eeuo pipefail

day=${1-}

if [ -z "$day" ]; then
	echo "Usage: $0 <day>" >&2
	exit 1
fi

# Find out the year for the challenge.
year=$(grep "AOC_YEAR" src/main.rs | cut -d = -f 2 | sed 's/;//g' | sed 's/ //g')
root="$(pwd)/src"
year=2022

dayname="day$day"
# Func name can be the second argument, but default to the dayname.
func_name=${2-day_$day}

daydir="$root/y$year"
dayfilename="$daydir/$dayname.rs"

if [ -f "$dayfilename" ]; then
	echo "Day file already exists for $day." >&2
	exit 1
fi

mkdir -p "$daydir"
touch "$dayfilename"

# Set up the basic file:
sed "s/day_replace/$func_name/g" bin/blankday.rs > "$dayfilename"

# Add to mod.rs.
mod_file="$daydir/mod.rs"

if grep "$dayname" "$mod_file"; then
	echo "Day already added to mod.rs" >&2
	exit 1
fi


### IMPORTANT: -i '' and [[:space:]] are for macOS compatibility and do not work on Linux D:

# Firstly, insert mod statement before "run_day"
mod_code="mod $dayname;"
sed -i '' "s/^[[:space:]]*pub fn run_day/$mod_code\n&/" "$mod_file"

# Secondly, insert case code before "_ =>"
case_code="        $day => $dayname::$func_name(input),"
sed -i '' "s/^[[:space:]]*_ =/$case_code\n&/" "$mod_file" 