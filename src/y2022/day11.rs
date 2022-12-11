use regex::Regex;

// Note: u128 was used when I was testing and getting number overflow errors. It's
// likely not needed now.
#[derive(Debug, Clone)]
struct Monkey {
    items_worry: Vec<u128>,
    operation: String,
    divisible_by: u128,
    true_to: u128,
    false_to: u128,
    monkey_num: u128,
    times_inspected: u128,
}

pub fn day_11(input: &String, part_two: bool) -> String {
    // let input = "Monkey 0:
    //     Starting items: 79, 98
    //     Operation: new = old * 19
    //     Test: divisible by 23
    //       If true: throw to monkey 2
    //       If false: throw to monkey 3

    //   Monkey 1:
    //     Starting items: 54, 65, 75, 74
    //     Operation: new = old + 6
    //     Test: divisible by 19
    //       If true: throw to monkey 2
    //       If false: throw to monkey 0

    //   Monkey 2:
    //     Starting items: 79, 60, 97
    //     Operation: new = old * old
    //     Test: divisible by 13
    //       If true: throw to monkey 1
    //       If false: throw to monkey 3

    //   Monkey 3:
    //     Starting items: 74
    //     Operation: new = old + 3
    //     Test: divisible by 17
    //       If true: throw to monkey 0
    //       If false: throw to monkey 1
    //   ";

    let mut monkeys: Vec<Monkey> = input_to_monkeys(&input.to_string());

    for monkey in &monkeys {
        println!("Monkey: {:?}", monkey);
    }

    let decrease_by = if part_two {
        monkeys
            .iter()
            .fold(1, |acc, monkey| monkey.divisible_by * acc)
    } else {
        3
    };

    let num_rounds = if part_two { 10000 } else { 1 };
    for round in 0..num_rounds {
        println!("ROUND: {}", round);
        for i in 0..monkeys.len() {
            // Get a clone for local read access.
            let monkey = monkeys[i].clone();
            for item in monkey.items_worry {
                println!("Monkey inspects an item: {}", item);
                // Reference the main monkey array, not the clone.
                monkeys[i].times_inspected += 1;

                // Increase worry as inspecting.
                let mut new_worry = increase_worry(item, monkey.operation.clone());

                println!("  worry level increased to: {}", new_worry);

                // Decrease worry as undamaged.
                if part_two {
                    new_worry = new_worry % decrease_by;
                } else {
                    // Round down division.
                    new_worry = (new_worry - (new_worry % decrease_by)) / decrease_by;
                }

                println!("  worry level decreased to: {}", new_worry);

                if new_worry % monkey.divisible_by == 0 {
                    let true_to = monkey.true_to as usize;
                    monkeys[true_to].items_worry.push(new_worry);
                    println!("  DIVISIBLE: thrown to: {}", true_to);
                } else {
                    let false_to = monkey.false_to as usize;
                    monkeys[false_to].items_worry.push(new_worry);
                    println!("  NOTDIV: thrown to: {}", false_to);
                }
            }
            // The monkey now has no items since all have been thrown to another monkey.
            monkeys[i].items_worry = Vec::new();
        }
    }

    // In-place sort by times inspected.
    monkeys.sort_by(|a, b| a.times_inspected.cmp(&b.times_inspected));

    for monkey in monkeys.iter() {
        println!(
            "Monkey: {} inspected items {} times.",
            monkey.monkey_num, monkey.times_inspected
        );
    }

    let monkey_business =
        monkeys[monkeys.len() - 1].times_inspected * monkeys[monkeys.len() - 2].times_inspected;

    return monkey_business.to_string();
}

fn increase_worry(worry: u128, operation: String) -> u128 {
    let mut op_parts = operation.split_whitespace();

    let first_param = match op_parts.next() {
        Some("old") => worry,
        Some(num) => num.parse::<u128>().unwrap(),
        _ => panic!("Invalid operation param"),
    };

    let operator = op_parts.next().unwrap();

    println!("Doing op: {}. With value: {}", operation, worry);
    let last_param = match op_parts.next() {
        Some("old") => worry,
        Some(num) => num.parse::<u128>().unwrap(),
        _ => panic!("Invalid operation param"),
    };

    match operator {
        "+" => first_param + last_param,
        "*" => first_param * last_param,
        _ => panic!("Invalid operator"),
    }
}

// Parse the monkeys using regex;
fn input_to_monkeys(input: &String) -> Vec<Monkey> {
    input
        .split("\n\n")
        .filter_map(|str| {
            if str.is_empty() {
                return None;
            }
            let monkey_reg =
                Regex::new(r"Monkey (.*):\n.*Starting items: (.*)\n.*Operation: new = (.*)\n.*Test: divisible by (.*)\n.*If true: throw to monkey (.*)\n.*If false: throw to monkey (.*)")
                    .unwrap();
            let reg_cap = monkey_reg.captures(str).unwrap();

            let items_worry: Vec<u128> = reg_cap
                .get(2)
                .unwrap()
                .as_str()
                .split(',')
                .map(|num| num.trim().parse::<u128>().unwrap())
                .collect();

            Some(Monkey {
                items_worry,
                monkey_num: num_from_capture(reg_cap.get(1)),
                operation: reg_cap.get(3).unwrap().as_str().to_string(),
                divisible_by: num_from_capture(reg_cap.get(4)),
                true_to: num_from_capture(reg_cap.get(5)),
                false_to: num_from_capture(reg_cap.get(6)),
                times_inspected: 0,
            })
        })
        .collect()
}

fn num_from_capture(capture: Option<regex::Match>) -> u128 {
    capture.unwrap().as_str().parse::<u128>().unwrap()
}
