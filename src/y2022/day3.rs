use std::collections::HashMap;
use std::collections::HashSet;

pub fn rucksack_calculations(input: &String, part_two: bool) -> String {
    if part_two {
        get_item_badge_priorities(input)
    } else {
        get_split_item_priorities(input)
    }
}

// Gets the score of the badges (and their priorities) for each elf group.
fn get_item_badge_priorities(input: &String) -> String {
    // Separate the input into groups of three elves.
    let all_elves = input.lines().collect::<Vec<_>>();
    let elf_groups = all_elves.chunks(3);

    elf_groups
        .fold(0, |priority_sums, group| {
            // We want to find the single item shared between all three elves. We
            // can do that by intersecting A & B, and then intersecting the result
            // with C.

            // Do some weird conversions from a vector of chars to String (and then &str).
            let first_two = intersection(group[0], group[1])
                .into_iter()
                .collect::<String>();

            // The intersection of (the intersection of the first two) and three is the intersection of all three.
            let chars_in_all = intersection(first_two.as_str(), group[2]);

            // Finally, we score the characters.
            priority_sums + score_chars(chars_in_all)
        })
        .to_string()
}

// Gets the score of the items split into both rucksacks.
fn get_split_item_priorities(input: &String) -> String {
    // Each new line of input represents a rucksack.
    input
        .lines()
        .fold(0, |priority_sums, items| {
            if items.is_empty() {
                return priority_sums;
            }
            // One half of the line is one compartment, the other half is the other compartment.
            let (compartment_a, compartment_b) = items.split_at(items.len() / 2);

            // Find each character shared in both rucksack compartments.
            let chars_in_both = intersection(compartment_a, compartment_b);

            println!("Shared Items in strs: {}-{}", compartment_a, compartment_b);

            // Sum the "priorities" of those characters.
            priority_sums + score_chars(chars_in_both)
        })
        .to_string()
}

// Basic wrapper to score a vector of characters, which is the output of the intersection function.
fn score_chars(c: Vec<char>) -> u32 {
    c.iter().fold(0, |score, c| score + char_to_score(*c))
}

// Use Ascii trickery to get the score of a character.
// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn char_to_score(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 38
    } else {
        c as u32 - 96
    }
}

// Returns a list of characters contained in both strings.
fn intersection(a: &str, b: &str) -> Vec<char> {
    // Map of characters to number of occurrences in the string.
    let mut char_map = HashMap::new();

    // Add everything in a to the map.
    for c in a.chars() {
        char_map
            .entry(c)
            .and_modify(|occurrences| *occurrences += 1)
            .or_insert(1);
    }

    // Create a set of characters that are in both strings.
    let mut chars_in_both = HashSet::new();
    for c in b.chars() {
        if char_map.contains_key(&c) {
            chars_in_both.insert(c);
        }
    }

    return chars_in_both.into_iter().collect();
}
