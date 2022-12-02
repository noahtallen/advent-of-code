pub fn elves_by_food(input: &String) -> String {
    let sections = input.split("\n");

    // Save food groupings.
    let mut elves_by_most_food: Vec<u64> = Vec::new();

    sections.fold(0, |acc: u64, individual_food: &str| {
        // If can't be parsed, assume 0 -- likely the empty string in that case.
        let caloric_value = match individual_food.parse::<u64>() {
            Ok(calories) => calories,
            Err(_) => 0,
        };

        let total_elf_food = acc + caloric_value;

        // When we reach an empty value, we've accumulated all the food for the
        // current elf. At this point, we can maintain a sorted list.
        if individual_food.is_empty() {
            let pos = elves_by_most_food
                .binary_search(&total_elf_food)
                .unwrap_or_else(|e| e);

            elves_by_most_food.insert(pos, total_elf_food);

            // Now, reset the accumulator for the next elf.
            return 0;
        }

        // Simply accumulating the food for the current elf.
        return total_elf_food;
    });

    // Grab the last three elves and print them.
    let top_three_elves = &elves_by_most_food[elves_by_most_food.len() - 3..];

    // Show the top three, which means reversing our little iterator.
    for (i, top_food) in top_three_elves.iter().rev().enumerate() {
        println!("Elf {}: {}", i + 1, top_food);
    }

    let total_top_food: u64 = top_three_elves.iter().sum();
    println!(
        "Elf with most food (P1): {}. Top three elves summed (P2): {}",
        top_three_elves[2], total_top_food
    );

    return total_top_food.to_string();
}
