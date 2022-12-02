pub mod constants;
pub mod helpers;

use std::fs;

use constants::Outcome;
use helpers::{compute_next_move, compute_outcome, convert_to_outcome};

fn main() {
    let strategy_guide = fs::read_to_string("./src/Assets/input.txt").unwrap();

    let rounds = strategy_guide.lines();

    let mut total_score = 0;
    let mut real_score = 0;

    for round in rounds {
        let in_out: Vec<&str> = round.split_whitespace().collect();
        if in_out.len() != 2 {
            panic!("corrupted input")
        }

        total_score += compute_score(&in_out);
        real_score += compute_real_score(&in_out)
    }

    println!("{total_score}");
    println!("{real_score}")
}

fn compute_real_score(in_out: &Vec<&str>) -> usize {
    let mut round_score: usize = 0;

    let enemy_choice = in_out[0];
    let action = in_out[1];

    let round_outcome = convert_to_outcome(action).unwrap();
    let action_to_perform = compute_next_move(enemy_choice, &round_outcome).unwrap();

    round_score += (action_to_perform as usize) + 1;
    match round_outcome {
        Outcome::WIN => round_score += 6,
        Outcome::DRAW => round_score += 3,
        Outcome::LOSE => {}
    }

    return round_score;
}

fn compute_score(in_out: &Vec<&str>) -> usize {
    let mut round_score = 0;

    let enemy_choice = in_out[0];
    let counter_choice = in_out[1];

    let (outcome, score) = compute_outcome(enemy_choice, counter_choice).unwrap();

    round_score += score;
    match outcome {
        Outcome::WIN => round_score += 6,
        Outcome::DRAW => round_score += 3,
        Outcome::LOSE => {}
    }

    return round_score;
}
