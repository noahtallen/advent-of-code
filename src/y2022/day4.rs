use std::ops::Range;

pub fn assignment_checker(input: &String, part_two: bool) -> String {
    input
        .lines()
        .fold(0, |num_overlapping, elf_pair| {
            // Each line contains assignments for a pair of elves.
            let (elf_a, elf_b) = elf_pair.rsplit_once(',').unwrap();
            // Each of those assignments represents a range of numbers.
            let (elf_a_range, elf_b_range) = (elf_range(elf_a), elf_range(elf_b));

            // We need to determine if the current pair of ranges overlaps.
            let has_overlap: bool = if part_two {
                range_overlap(&elf_a_range, &elf_b_range)
            } else {
                range_contains(&elf_a_range, &elf_b_range)
            };

            return num_overlapping + has_overlap as u32;
        })
        .to_string()
}

// Extends the range to allow calculations of "overlap" and "range contains"
pub trait RangeExt<T> {
    fn contains_range(&self, other: &Range<T>) -> bool;
    fn overlaps_range(&self, other: &Range<u32>) -> bool;
}

impl RangeExt<u32> for Range<u32> {
    // True if the other range is entirely within the bounds of this range.
    fn contains_range(&self, other: &Range<u32>) -> bool {
        // Start and end are both beyond the bounds of the other range.
        self.start <= other.start && self.end >= other.end
    }

    // True if either the start or end is within the bounds of the other range.
    // E.g. true if there is a partial range overlap.
    fn overlaps_range(&self, other: &Range<u32>) -> bool {
        (other.start <= self.start && self.start <= other.end) // Start within bounds.
            || (other.start <= self.end && self.end <= other.end) // End within bounds.
    }
}

fn range_overlap(range_a: &Range<u32>, range_b: &Range<u32>) -> bool {
    range_a.overlaps_range(range_b) || range_b.overlaps_range(range_a)
}

fn range_contains(range_a: &Range<u32>, range_b: &Range<u32>) -> bool {
    range_a.contains_range(range_b) || range_b.contains_range(range_a)
}

// Converts a range string (e.g. "1-3") into an actual range type.
fn elf_range(str_range: &str) -> Range<u32> {
    let numbers: Vec<u32> = str_range
        .split('-')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    numbers[0]..numbers[1]
}
