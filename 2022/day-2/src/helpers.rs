use crate::constants::{Action, Option, Outcome, ENEMY_POSSIBLE_CHOICES, MY_POSSIBLE_CHOICES};

pub fn compute_next_move(enemy_choice: &str, round_outcome: &Outcome) -> Option<Action> {
    return match enemy_choice {
        "A" => match round_outcome {
            Outcome::WIN => Option::Ok(Action::Paper),
            Outcome::DRAW => Option::Ok(Action::Rock),
            Outcome::LOSE => Option::Ok(Action::Scissor),
        },
        "B" => match round_outcome {
            Outcome::WIN => Option::Ok(Action::Scissor),
            Outcome::DRAW => Option::Ok(Action::Paper),
            Outcome::LOSE => Option::Ok(Action::Rock),
        },
        "C" => match round_outcome {
            Outcome::WIN => Option::Ok(Action::Rock),
            Outcome::DRAW => Option::Ok(Action::Scissor),
            Outcome::LOSE => Option::Ok(Action::Paper),
        },
        _ => Option::None,
    };
}

pub fn compute_outcome(enemy_choice: &str, mut my_choice: &str) -> Option<(Outcome, usize)> {
    if !ENEMY_POSSIBLE_CHOICES.contains(&enemy_choice) || !MY_POSSIBLE_CHOICES.contains(&my_choice)
    {
        return Option::None;
    }

    let score = MY_POSSIBLE_CHOICES
        .iter()
        .position(|&x| x == my_choice)
        .unwrap()
        + 1;

    my_choice = convert_bases(my_choice).unwrap();
    if my_choice == enemy_choice {
        return Option::Ok((Outcome::DRAW, score));
    }

    if (my_choice == "A" && enemy_choice == "C")
        || (my_choice == "B" && enemy_choice == "A")
        || (my_choice == "C" && enemy_choice == "B")
    {
        return Option::Ok((Outcome::WIN, score));
    }

    return Option::Ok((Outcome::LOSE, score));
}

pub fn convert_to_outcome(choice: &str) -> Option<Outcome> {
    return match choice {
        "X" => Option::Ok(Outcome::LOSE),
        "Y" => Option::Ok(Outcome::DRAW),
        "Z" => Option::Ok(Outcome::WIN),
        _ => Option::None,
    };
}

pub fn convert_bases(choice: &str) -> Option<&str> {
    if ENEMY_POSSIBLE_CHOICES.contains(&choice) {
        return match choice {
            "A" => Option::Ok("X"),
            "B" => Option::Ok("Y"),
            "C" => Option::Ok("Z"),
            _ => Option::None,
        };
    }
    if MY_POSSIBLE_CHOICES.contains(&choice) {
        return match choice {
            "X" => Option::Ok("A"),
            "Y" => Option::Ok("B"),
            "Z" => Option::Ok("C"),
            _ => Option::None,
        };
    }
    return Option::None;
}
