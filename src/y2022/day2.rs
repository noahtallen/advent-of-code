enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

fn compute_score(outcome: &Outcome, my_choice: &RPS) -> i32 {
    let outcome_score = match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Loss => 0,
    };

    let choice_score = match my_choice {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };

    return outcome_score + choice_score;
}

fn compute_outcome(me: &RPS, opponent: &RPS) -> Outcome {
    match (me, opponent) {
        (RPS::Rock, RPS::Paper) => Outcome::Loss,
        (RPS::Rock, RPS::Scissors) => Outcome::Win,
        (RPS::Paper, RPS::Rock) => Outcome::Win,
        (RPS::Paper, RPS::Scissors) => Outcome::Loss,
        (RPS::Scissors, RPS::Rock) => Outcome::Loss,
        (RPS::Scissors, RPS::Paper) => Outcome::Win,
        _ => Outcome::Draw,
    }
}

fn get_choice_from_outcome(outcome: &Outcome, other_choice: &RPS) -> RPS {
    match (other_choice, outcome) {
        (RPS::Rock, Outcome::Win) => RPS::Paper,
        (RPS::Rock, Outcome::Loss) => RPS::Scissors,
        (RPS::Rock, Outcome::Draw) => RPS::Rock,

        (RPS::Scissors, Outcome::Win) => RPS::Rock,
        (RPS::Scissors, Outcome::Loss) => RPS::Paper,
        (RPS::Scissors, Outcome::Draw) => RPS::Scissors,

        (RPS::Paper, Outcome::Win) => RPS::Scissors,
        (RPS::Paper, Outcome::Loss) => RPS::Rock,
        (RPS::Paper, Outcome::Draw) => RPS::Paper,
    }
}

fn get_my_choice(me: &str, opponent: &RPS, is_part_two: bool) -> RPS {
    if is_part_two {
        let desired_outcome = match me {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid desired outcome"),
        };

        return get_choice_from_outcome(&desired_outcome, opponent);
    }

    match me {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => panic!("Invalid my choice"),
    }
}

// Goal: compute my total score in rock paper scissors by assesing the outcome of each round.
pub fn get_score_from_elf_rps_game(input: &String, part_two: bool) -> String {
    input
        .split("\n")
        .fold(0, |score_so_far, line| {
            // Skip empty lines.
            if line.is_empty() {
                return score_so_far;
            }

            // First, match the input to the rock, paper, scissors enum.
            let choices: Vec<&str> = line.split(" ").collect();
            let opponent_choice = match choices[0] {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissors,
                _ => panic!("Invalid opponent choice"),
            };

            // Which choice to use is dependent on the part we're solving, so let's split it out.
            let my_choice = get_my_choice(choices[1], &opponent_choice, part_two);

            // Next, compute various data about the game, including the score.
            let outcome = compute_outcome(&my_choice, &opponent_choice);
            let score = compute_score(&outcome, &my_choice);

            return score_so_far + score;
        })
        .to_string()
}
