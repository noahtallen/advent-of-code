pub fn signal_strength(input: &String, _part_two: bool) -> String {
    // A queue of commands to process, in iterator form.
    let mut cmds = input.lines();

    // A list of values we've looked at along the way, for part one.
    let mut tracked_signals: Vec<i32> = Vec::new();

    let mut cycle_num = 0;

    // A 64 bit register.
    let mut register_x = 1i32;

    // A pending command which needs to be finalized in a future tick.
    let mut command_start_cycle = 0;
    let mut active_command: Option<i32> = None;

    // One while loop iteration is one CPU tick. Loop indefinitely until the command
    // queue is empty. Since a command can take multiple ticks, we can't just
    // iterate over the commands.
    #[allow(while_true)]
    while true {
        cycle_num += 1;

        // Very first, finalize pending commands (if any.)
        if let Some(active_cmd) = active_command {
            if cycle_num - command_start_cycle == 2 {
                register_x += active_cmd;
                active_command = None;
            }
        }

        // Next, we can do things "during" the CPU cycle, such as check the register
        // value or draw the sprite.

        if cycle_num == 20 || (cycle_num - 20) % 40 == 0 {
            let signal = register_x * cycle_num;
            tracked_signals.push(signal);
        }

        draw_sprite(register_x, cycle_num);

        // Do not continue to the command parser if there is an active command.
        // Otherwise, we'd consume the command queue too quickly.
        if active_command.is_some() {
            continue;
        }

        // Process the next command string.
        let mut cmd = match cmds.next() {
            Some(cmd) => cmd.trim().split_whitespace(),
            None => {
                // Stop the CPU when the command queue is empty.
                break;
            }
        };

        match cmd.next() {
            Some("addx") => {
                let add_by = cmd.next().unwrap().parse::<i32>().unwrap();
                command_start_cycle = cycle_num;
                active_command = Some(add_by);
                // do something in two ticks.
            }
            Some("noop") => continue,
            other_cmd => println!(
                "Unsupported command name: {}",
                other_cmd.unwrap_or("undefined")
            ),
        }
    }

    println!("\nCycles: {}", cycle_num);
    tracked_signals.iter().sum::<i32>().to_string()
}

fn draw_sprite(sprite_pos: i32, cycle: i32) {
    if cycle == 1 || (cycle - 1) % 40 == 0 {
        print!("\n");
    }

    // Positions are 0 indexed, but cycles are 1 indexed. So we need to adjust slightly.
    let pixel_pos = (cycle - 1) % 40;

    // Is visible if position within one of the pixel position.
    if sprite_pos - 1 <= pixel_pos && pixel_pos <= sprite_pos + 1 {
        print!("#");
    } else {
        print!(".");
    }
}
