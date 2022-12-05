use std::ops::Range;

pub fn assignment_checker(input: &String, part_two: bool) -> String {
    input
        .lines()
        .fold(0, |num_overlapping, elf_pair| {
            // Each line contains assignments for a pair of elves. Each of those
            // assignments represents a range of numbers.
            let (elf_a_range, elf_b_range) = input_to_range(elf_pair);

            // Determine if the current pair of ranges overlaps according to the challenge rules.
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
fn input_to_range(range_pair: &str) -> (Range<u32>, Range<u32>) {
    // Tuple where each element is the elf range.
    let (elf_a, elf_b) = range_pair.rsplit_once(',').unwrap();

    // Tuple where each element is the start and end of the range.
    let (a_start, a_end) = elf_a.rsplit_once('-').unwrap();
    let (b_start, b_end) = elf_b.rsplit_once('-').unwrap();

    // Convert the strings to numbers and then to a range:
    let a_range = a_start.parse::<u32>().unwrap()..a_end.parse::<u32>().unwrap();
    let b_range = b_start.parse::<u32>().unwrap()..b_end.parse::<u32>().unwrap();
    (a_range, b_range)
}
