use std::process;

#[derive(PartialEq)]
enum FS {
    DIR,
    FILE,
}

struct FileSystemEntry {
    name: String,
    size: Option<u64>,
    kind: FS,
    children: Vec<usize>,
    parent: Option<usize>,
}

pub fn directory_sizes(input: &String, part_two: bool) -> String {
    // Create the root of the filesystem!
    let mut fs_tree: Vec<FileSystemEntry> = Vec::new();
    add_entry(&mut fs_tree, None, "/".to_string(), FS::DIR, None);

    let mut active_dir: Option<usize> = None;

    // Each command starts with $ and is followed by some number of output, so we
    // can process all of that command text at once.
    for cmd_group in input.split_terminator("$ ") {
        if cmd_group.is_empty() {
            continue;
        }

        // Prepare the command data for processing. Essentially, the command is
        // with a whitespace deliminated string, and the output is a vector of
        let mut cmd_vec: Vec<&str> = cmd_group.lines().collect();
        let output = cmd_vec.split_off(1);
        let mut cmd_data = cmd_vec[0].split_whitespace();

        // Executes the commands and arguments, mostly just adding to the list
        match cmd_data.next() {
            // Cd is very simple: just re-arrange the active directory pointer.
            // Note that cd will not work until ls runs, unless it's "cd /". This
            // is because the filesystem can only be populated as ls runs.
            Some("cd") => {
                match cmd_data.next() {
                    Some("..") => {
                        if let Some(dir) = active_dir {
                            active_dir = fs_tree[dir].parent;
                        }
                    }
                    Some("/") => {
                        active_dir = Some(0);
                    }
                    Some(entry_name) => {
                        active_dir = find_entry(&fs_tree, active_dir, &entry_name.to_string())
                    }
                    None => {
                        // This should never happen given the input contstraints.
                        println!("cd needs a directory name!");
                        process::exit(1);
                    }
                };
            }
            Some("ls") => {
                // ls does the same thing each time. Parse through all the output
                // and add the entries to the tree.
                for entry in output {
                    let mut parts = entry.split_whitespace();
                    // Entries either start with "dir" or the size if it's a file.
                    match parts.next() {
                        Some("dir") => {
                            add_entry(
                                &mut fs_tree,
                                active_dir,
                                parts.next().unwrap().to_string(),
                                FS::DIR,
                                None,
                            );
                        }
                        Some(size) => {
                            add_entry(
                                &mut fs_tree,
                                active_dir,
                                parts.next().unwrap().to_string(),
                                FS::FILE,
                                Some(size.parse::<u64>().unwrap()),
                            );
                        }
                        None => (), // empty ls, I guess!
                    }
                }
            }
            Some(unknown_cmd) => println!("Unknown command: {}", unknown_cmd),
            None => (),
        }
    }

    // Now that we've populated the tree, update the directory sizes.
    let total_size = update_tree_with_sizes(&mut fs_tree, 0);
    print_tree(&fs_tree, 0, 0);

    if !part_two {
        return sum_dirs_smaller_than(&fs_tree, 100000).to_string();
    }

    // Hardcoded by the challenge:
    let fs_size = 70000000;
    let space_needed = 30000000;
    let find_space = space_needed - (fs_size - total_size);

    println!(
        "\nTotal size: {}. FS size: {}. Space remaining: {}. Space to find: {}",
        total_size,
        fs_size,
        fs_size - total_size,
        find_space
    );

    find_smallest_dir_bigger_than(&fs_tree, find_space).to_string()
}

// Updates the "tree" under the provided index by recursing through directories
// to find their ultimate sizes. Returns the size of the subtree. Starting at
// index 0 will return the size of the whole tree.
fn update_tree_with_sizes(tree: &mut Vec<FileSystemEntry>, index: usize) -> u64 {
    if index >= tree.len() {
        return 0;
    }

    let entry = &tree[index];
    if entry.kind == FS::DIR {
        tree[index].size = Some(
            entry
                .children
                .clone()
                .iter()
                .map(|c| update_tree_with_sizes(tree, *c))
                .sum(),
        );
    }

    // We cannot borrow "entry" any more, so we must access it directly.
    tree[index].size.unwrap()
}

// Returns the size of the smallest directory bigger than the provided size.
fn find_smallest_dir_bigger_than(tree: &Vec<FileSystemEntry>, size: u64) -> u64 {
    match tree
        .iter()
        .filter_map(|entry| {
            if entry.kind == FS::DIR && entry.size.unwrap_or(0) > size {
                Some(entry.size.unwrap_or(0))
            } else {
                None
            }
        })
        .min()
    {
        Some(min) => min,
        None => 0,
    }
}

// Returns the sum of all directories smaller than the provided size.
fn sum_dirs_smaller_than(tree: &Vec<FileSystemEntry>, size: u64) -> u64 {
    tree.iter()
        .filter_map(|e| {
            if e.kind == FS::DIR && e.size.unwrap_or(0) <= size {
                Some(e.size.unwrap_or(0))
            } else {
                None
            }
        })
        .sum()
}

// Prints the entire tree to the terminal.
fn print_tree(tree: &Vec<FileSystemEntry>, index: usize, depth: usize) {
    if index >= tree.len() {
        return;
    }
    let entry = &tree[index];

    println!(
        ">{:<width$} {}{} ({})",
        "",
        if entry.kind == FS::DIR { "--" } else { "" },
        entry.name,
        entry.size.unwrap_or(0),
        width = depth * 2,
    );

    for child in &entry.children {
        print_tree(tree, *child, depth + 1);
    }
}

// Adds an entry to the "tree" and returns its index.
fn add_entry(
    tree: &mut Vec<FileSystemEntry>,
    parent: Option<usize>,
    name: String,
    kind: FS,
    size: Option<u64>,
) -> usize {
    // If the entry is already a child of this parent, simply return its index
    // rather than duplicating it.
    match find_entry(tree, parent, &name) {
        Some(index) => return index,
        None => (),
    }

    tree.push(FileSystemEntry {
        name,
        size,
        kind,
        children: Vec::new(),
        parent: parent,
    });

    // Make sure the parent knows about its new child!
    let index = tree.len() - 1;
    if let Some(p) = parent {
        tree[p].children.push(index);
    }
    return index;
}

// Finds the index of a child entry matching the provided name.
fn find_entry(tree: &Vec<FileSystemEntry>, parent: Option<usize>, name: &String) -> Option<usize> {
    if let Some(p) = parent {
        // TODO: This could be optimized (maybe with HashSet) to avoid the loop,
        // which needs to run fairly frequently.
        return match tree[p].children.iter().find(|c| tree[**c].name == *name) {
            Some(c) => Some(*c),
            None => None,
        };
    }

    None
}
