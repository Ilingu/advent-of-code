use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let total_chars = input.lines().map(|l| l.len()).sum::<usize>();

    let p1 = total_chars - input.lines().map(|l| memory_len(l)).sum::<usize>();
    println!("{p1}");

    let p2 = input.lines().map(|l| encoded_len(l)).sum::<usize>() - total_chars;
    println!("{p2}");
}

fn encoded_len(string: &str) -> usize {
    let re = Regex::new(r"(\\x)").unwrap();
    string.len()
        + 4
        + string.matches("\\").count() * 2
        + string.matches("\"").count() * 2
        + re.captures_iter(string).count()
}

fn memory_len(string: &str) -> usize {
    let chars = string.chars().collect::<Vec<_>>();
    let (mut char_idx, mut len) = (0, 0);
    while !chars.is_empty() && char_idx <= chars.len() - 1 {
        let ch = chars[char_idx];
        match ch {
            '\\' => match char_idx + 1 <= chars.len() - 1 {
                true => {
                    len += 1;
                    match chars[char_idx + 1] {
                        'x' => char_idx += 4,
                        _ => char_idx += 2,
                    }
                }
                false => break,
            },
            _ => {
                len += 1;
                char_idx += 1
            }
        }
    }
    len - 2
}
