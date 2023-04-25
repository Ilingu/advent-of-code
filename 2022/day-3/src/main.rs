use std::fs;

fn main() {
    let raw_rucksacks = fs::read_to_string("./src/Assets/input.txt").unwrap();

    let rucksacks = Vec::from_iter(raw_rucksacks.lines());

    let mut p1 = 0;
    for rucksack in rucksacks.clone() {
        let middle_index = (rucksack.len() / 2) - 1;

        let first_part = &rucksack[..middle_index + 1];
        let second_part = &rucksack[middle_index + 1..];

        let letter = find_common_letter(vec![first_part, second_part]).unwrap();

        p1 += letter_score(letter)
    }

    let mut p2 = 0;

    /* first approach of spliting by 3
    let mut groups_rucksacks: Vec<Vec<&str>> = vec![vec![""; 3]; rucksacks.len() / 3];
    let mut j = 0;
    for i in 0..rucksacks.len() {
        let k = i % 3;
        if k == 0 && i != 0 {
            j += 1;
        }

        groups_rucksacks[j][k] = rucksacks[i];
    }
    */

    // One liner way of spliting by 3
    let groups_rucksacks = rucksacks
        .chunks(3)
        .map(|c| Vec::from(c))
        .collect::<Vec<Vec<&str>>>();

    for group in groups_rucksacks {
        let letter = find_common_letter(group).unwrap();
        p2 += letter_score(letter)
    }

    println!("{p1}");
    println!("{p2}")
}

fn find_common_letter(parts: Vec<&str>) -> Option<u8> {
    let part_bytes = parts.first().unwrap().as_bytes();
    for char in part_bytes {
        let mut common = true;
        for compare in &parts[1..] {
            if !compare.as_bytes().contains(char) {
                common = false
            }
        }

        if common {
            return Option::Ok(*char);
        }
    }
    return Option::None;
}

const LETTER_SCORE: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn letter_score(char: u8) -> usize {
    LETTER_SCORE
        .as_bytes()
        .iter()
        .position(|&x| x == char)
        .unwrap()
        + 1
}

pub enum Option<T> {
    Ok(T),
    None,
}

impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(t) => t,
            Self::None => panic!("invalid result"),
        }
    }
}
