use std::{fs, vec};

fn main() {
    let datastream_buffer = fs::read_to_string("./src/input.txt").unwrap();

    let mut last_non_eq_chars: Vec<char> = vec![];
    let stream_chars = datastream_buffer.chars().collect::<Vec<char>>();

    let mut p1 = 0;
    let mut p2 = 0;

    for i in 0..stream_chars.len() {
        let char = stream_chars[i];
        if last_non_eq_chars.contains(&char) {
            last_non_eq_chars = vec![];
            continue;
        }

        last_non_eq_chars.push(char);
        if last_non_eq_chars.len() == 4 && p1 == 0 {
            p1 = i + 1;
        }
        if last_non_eq_chars.len() == 14 && p2 == 0 {
            p2 = i + 1;
            break;
        }
    }

    println!("{p1}");
    println!("{p2}")
}
