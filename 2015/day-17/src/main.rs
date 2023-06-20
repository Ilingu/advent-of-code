use std::fs;

use itertools::Itertools;

const EGGNOG_VOLUME: usize = 150;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let containers = input
        .lines()
        .map(|container| container.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut p1 = 0;
    let mut ways_per_containers: Vec<usize> = vec![0; containers.len()];

    for i in 0..=(containers.len()) {
        for comb in containers.iter().combinations(i) {
            if comb.iter().map(|x| **x).sum::<usize>() == EGGNOG_VOLUME {
                p1 += 1;
                ways_per_containers[i] += 1;
            }
        }
    }
    let p2 = ways_per_containers.iter().find(|x| x != &&0).unwrap();

    println!("{p1}");
    println!("{p2}")
}
