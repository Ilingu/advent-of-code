pub mod cave;

use std::fs;

use cave::Cave;

fn main() {
    let scans = fs::read_to_string("./Assets/input.txt").unwrap();
    let mut cave = Cave::build_cave(scans.as_str());

    let p1 = cave.fill_cave();
    println!("{p1}");

    cave.is_part_2 = true;
    let p2 = cave.fill_cave() + 1;
    println!("{p2}");
}
