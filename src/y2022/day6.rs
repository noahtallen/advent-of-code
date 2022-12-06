use std::collections::HashSet;

pub fn fix_comms_device(input: &String, part_two: bool) -> String {
    // In part one, the num of unique chars required is 4. In part two, it's 14.
    let chars_to_check = if part_two { 14 } else { 4 };

    // This primes the starter vec with the first few chars so that the main loop
    // below can start by checking for duplicates. The minus one means that in part
    // one, this vector is only 3 characters. With one pushed at the start of the
    // loop, it becomes 4.
    let (starter_chars, rest) = input.lines().next().unwrap().split_at(chars_to_check - 1);

    // The answer is the first iteration this vec contains unique characters.
    let mut last_four_chars = starter_chars.chars().collect::<Vec<_>>();

    for (position, c) in rest.chars().enumerate() {
        last_four_chars.push(c);

        if !are_duplicated_letters(&last_four_chars) {
            // Our iteration is offset by the number of chars we collected into
            // the starter vec, so add it. (This also ultimately accounts for the
            // answer being 1-indexed rather than 0-indexed.)
            return (position + chars_to_check).to_string();
        }

        // Update the chars with the new stuff to check.
        last_four_chars.remove(0);
    }

    return "Did not find non-duplicated groups...".to_string();
}

// This function looks like O(chars.len()), but since that is always 4, it's constant time.
fn are_duplicated_letters(chars: &Vec<char>) -> bool {
    // Collect the chars into a HashSet, which will remove duplicates.
    let set = chars.iter().collect::<HashSet<_>>();

    // If the set is less than the chars, there were duplicates.
    return set.len() < chars.len();
}
