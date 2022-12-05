use regex::Regex;

pub fn crate_rearrangement(input: &String, part_two: bool) -> String {
    // An empty line separates the two sections of the input.
    let mut input_sections = input.split("\n\n");

    let stack_str = input_sections.next().unwrap();
    println!("Input stack: \n{}", stack_str);

    // The data model is a vector of "stacks." Each stack is a vector of chars.
    // stack[0] is then the first stack of crates to rearrange. stack[0].pop()
    // would remove the top crate (represented by a character)
    let mut stacks = create_stacks(stack_str);
    println!("\nParsed stacks:");
    print_stack(&stacks);

    let commands = input_sections.next().unwrap();
    execute_instructions(commands, &mut stacks, part_two);
    println!("\nStacks after rearranging:");
    print_stack(&stacks);

    // For each stack, get the top box and put it in the string.
    let top_boxes: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    return top_boxes;
}

fn print_stack(stacks: &Vec<Vec<char>>) {
    for (i, stack) in stacks.iter().enumerate() {
        println!("Stack {}: {:?}", i + 1, stack);
    }
}

fn create_stacks(input: &str) -> Vec<Vec<char>> {
    // Change the string to a 2D vector of characters, which makes further iteration easier.
    let mut char_matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        char_matrix.push(line.chars().collect());
    }

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let width = char_matrix[0].len(); // All rows are the same width.
    let height = char_matrix.len();

    // Iterate through each **column**, starting at the bottom.
    for w in 0..width {
        // We only want to parse columns of text which begin with a number. So
        // if we cannot parse it to a number, we'll skip this column, as it doesn't
        // contain data we need. Otherwise, we'll continue.
        let stack_number = match char_matrix[height - 1][w].to_digit(10) {
            Some(n) => n as usize,
            None => continue,
        };

        stacks.push(Vec::new());

        // Height minus one skips the "start" of the bottom of the column, which is
        // where the number lives. Going in reverse lets us read from the bottom
        // up, so that the stack is in the correct order.
        for h in (0..height - 1).rev() {
            // No need to parse empty characters.
            if char_matrix[h][w] == ' ' {
                continue;
            }
            stacks[stack_number - 1].push(char_matrix[h][w]);
        }
    }
    return stacks;
}

// Execute each command one-by-one.
fn execute_instructions(instructions: &str, stacks: &mut Vec<Vec<char>>, part_two: bool) {
    for cmd_str in instructions.lines() {
        do_command(str_to_command(cmd_str), stacks, part_two);
    }
}

fn do_command(command: Command, stacks: &mut Vec<Vec<char>>, part_two: bool) {
    // This method quickly moves the "group" of moved boxes from one vector to the next.
    if part_two {
        let move_start = stacks[command.from - 1].len() - command.count;
        let boxes_to_move = stacks[command.from - 1].split_off(move_start);

        stacks[command.to - 1].extend_from_slice(boxes_to_move.as_slice());

        return;
    }

    // This method allows us to reverse the order of the moved items.
    for _ in 0..command.count {
        let item = match stacks[command.from - 1].pop() {
            Some(item) => item,
            None => continue, // If there aren't enough items, I guess we keep going.
        };

        // Since everything is 0-indexed, we need to subract one.
        stacks[command.to - 1].push(item);
    }
}

// Input is in the form of "move A from B to C", where A, B, and C are ints.
fn str_to_command(command: &str) -> Command {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let captures = re.captures(command).unwrap();

    // Not super safe, but we do know the input. If one of these isn't a number,
    // the correct behavior is to exit the program.
    let count = captures.get(1).unwrap().as_str().parse::<u64>().unwrap() as usize;
    let from = captures.get(2).unwrap().as_str().parse::<u64>().unwrap() as usize;
    let to = captures.get(3).unwrap().as_str().parse::<u64>().unwrap() as usize;

    return Command { from, to, count };
}

struct Command {
    from: usize,
    to: usize,
    count: usize,
}
