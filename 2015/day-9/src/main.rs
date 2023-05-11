use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    let mut costs = HashMap::new();
    let mut cities = HashSet::new();

    for l in input.lines() {
        let datas = l.split_whitespace().collect::<Vec<_>>();
        let (city_a, city_b, cost) = (datas[0], datas[2], datas[4]);
        costs.insert((city_a, city_b), cost.parse::<usize>().unwrap());
        costs.insert((city_b, city_a), cost.parse::<usize>().unwrap());
        cities.insert(city_a);
        cities.insert(city_b);
    }

    let mut p1 = None;
    let mut p2 = 0;
    for tp in cities.iter().permutations(cities.len()) {
        let mut total_cost = 0;
        for idx in 0..tp.len() - 1 {
            let (&city_a, &city_b) = (tp[idx], tp[idx + 1]);
            let cost = costs[&(city_a, city_b)];
            total_cost += cost;
        }
        p1 = Some(total_cost.min(if p1.is_none() {
            total_cost
        } else {
            p1.unwrap()
        }));
        p2 = total_cost.max(p2);
    }
    println!("{}", p1.unwrap());
    println!("{p2}");
}
