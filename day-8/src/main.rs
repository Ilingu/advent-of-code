pub mod forest;

use std::fs;

use forest::Forest;

fn main() {
    let raw_map = fs::read_to_string("./Assets/input.txt").unwrap();

    let map = Forest::from_input(raw_map);
    // println!("{:#?}", map);

    let p1 = map.count_visible();
    let p2 = map.compute_scenic_score();
    println!("{p1}");
    println!("{p2}");
}
