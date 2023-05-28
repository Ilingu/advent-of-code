use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    // parsing input
    let mut people = HashSet::new();
    let mut hapiness_relationship = HashMap::new();

    for sentence in input.lines() {
        let datas = sentence.split_whitespace().collect::<Vec<_>>();
        let (from, hapiness, to) = (
            datas[0],
            match datas[2] {
                "lose" => -1,
                _ => 1,
            } * datas[3].parse::<isize>().unwrap(),
            datas[10].trim_end_matches('.'),
        );
        hapiness_relationship.insert((from, to), hapiness);
        people.insert(from);
    }

    // p1
    let mut p1 = 0;
    for arrangement in people.iter().permutations(people.len()) {
        p1 = p1.max(compute_hapiness(&arrangement, &hapiness_relationship));
    }
    println!("{p1}");

    // p2

    // Adding "me"
    for p in &people {
        hapiness_relationship.insert(("me", p), 0);
        hapiness_relationship.insert((p, "me"), 0);
    }
    people.insert("me");

    let mut p2 = 0;
    for arrangement in people.iter().permutations(people.len()) {
        p2 = p2.max(compute_hapiness(&arrangement, &hapiness_relationship));
    }
    println!("{p2}");
}

fn compute_hapiness(
    arrangement: &[&&str],
    hapiness_relationship: &HashMap<(&str, &str), isize>,
) -> isize {
    let mut cycle = arrangement.to_vec();
    cycle.push(arrangement[0]);

    let mut total = 0;
    for i in 0..(cycle.len() - 1) {
        let (from, to) = (*cycle[i], *cycle[i + 1]);
        total += hapiness_relationship[&(from, to)] + hapiness_relationship[&(to, from)];
    }
    total
}
