use std::collections::HashSet;

pub fn check_tree_visibility(input: &String, _part_two: bool) -> String {
    // Example input, with expected result of 21.
    // let input = "30373
    //              25512
    //              65332
    //              33549
    //              35390";

    // Collect input to a 2d array of numbers.
    let trees: Vec<Vec<i16>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| match c.to_digit(10) {
                    Some(d) => Some(d as i16), // Have to use match to get the i16 type.
                    None => None, // Handle none so we can have spaces in the input, like above.
                })
                .collect()
        })
        .collect();

    println!("Tree size: {}x{}", trees.len(), trees[0].len());
    let seen_trees = get_visible_trees(&trees);
    seen_trees.len().to_string()
}

fn get_visible_trees(trees: &Vec<Vec<i16>>) -> HashSet<String> {
    let mut seen_trees: HashSet<String> = HashSet::new();

    let height = trees.len();
    let width = trees[0].len();

    // TODO: I'd really love to make this more generic, but I'm not sure how.
    for w in 0..width {
        let mut tallest_tree = -1;
        // Because Rust is VERY strict with memory access, we can't use it multiple
        // times in this width for loop. So we have to generate the ranges again.
        for h in 0..height {
            if add_tallest_trees(&mut seen_trees, &trees, h, w, &mut tallest_tree) {
                break;
            }
        }
        tallest_tree = -1;
        for h in (0..height).rev() {
            if add_tallest_trees(&mut seen_trees, &trees, h, w, &mut tallest_tree) {
                break;
            }
        }
    }
    for h in 0..height {
        let mut tallest_tree = -1;
        for w in 0..width {
            if add_tallest_trees(&mut seen_trees, &trees, h, w, &mut tallest_tree) {
                break;
            }
        }
        tallest_tree = -1;
        for w in (0..width).rev() {
            if add_tallest_trees(&mut seen_trees, &trees, h, w, &mut tallest_tree) {
                break;
            }
        }
    }

    return seen_trees;
}

fn add_tallest_trees(
    set: &mut HashSet<String>,
    trees: &Vec<Vec<i16>>,
    h: usize,
    w: usize,
    tallest_tree: &mut i16,
) -> bool {
    let tree = trees[h][w];
    if tree > *tallest_tree {
        *tallest_tree = tree;
        set.insert(format!("{},{}", h, w));
    }

    // 9 is the max height, so we should stop looking if we see it.
    tree >= 9
}
