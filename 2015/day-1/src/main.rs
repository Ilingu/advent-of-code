use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let (mut floor, mut p2) = (0, None);

    for (i, ch) in input.chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!(),
        }
        if floor == -1 && p2.is_none() {
            p2 = Some(i + 1);
        }
    }

    println!("{floor}"); // p1
    println!("{}", p2.unwrap()); // p2
}
