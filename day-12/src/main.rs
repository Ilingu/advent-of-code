pub mod heightmap;

use std::fs;

use heightmap::HeightMap;

fn main() {
    let input = fs::read_to_string("./Assets/input.txt").unwrap();
    let hm = HeightMap::parse(&input);

    let p1 = hm.shortest_path();
    let p2 = hm.shortest_a_path();

    println!("Part1: {p1}");
    println!("Part2: {p2}");
}
