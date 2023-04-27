use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let p1 = input
        .lines()
        .fold(0, |acc, x| if is_nice_p1(x) { acc + 1 } else { acc });
    println!("{p1}");

    let p2 = input
        .lines()
        .fold(0, |acc, x| if is_nice_p2(x) { acc + 1 } else { acc });
    println!("{p2}");
}

fn is_nice_p1(s: &str) -> bool {
    let (mut vowel_count, mut has_twice): (usize, bool) = (0, false);
    for (i, ch) in s.chars().enumerate() {
        if i + 1 <= s.len() - 1 && s.chars().nth(i + 1).unwrap() == ch {
            has_twice = true
        }

        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => {}
        }
    }

    vowel_count >= 3
        && has_twice
        && !s.contains("ab")
        && !s.contains("cd")
        && !s.contains("pq")
        && !s.contains("xy")
}

fn is_nice_p2(s: &str) -> bool {
    let mut pair_visited = HashMap::new();
    let (mut pair_twice, mut repeats) = (false, false);

    for (i, ch) in s.chars().enumerate() {
        if i + 2 <= s.len() - 1 && s.chars().nth(i + 2).unwrap() == ch {
            repeats = true
        }
        if i + 1 <= s.len() - 1 {
            let pair = &s[i..=(i + 1)];
            if pair_visited.contains_key(&pair) {
                if i - pair_visited[pair] > 1 {
                    pair_twice = true
                }
            } else {
                pair_visited.insert(pair, i);
            }
        }
    }

    pair_twice && repeats
}
