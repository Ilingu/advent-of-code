pub mod blueprints;

use std::fs;

use blueprints::Blueprints;

fn main() {
    let mut blueprints = fs::read_to_string("./Assets/input.txt")
        .unwrap()
        .lines()
        .map(|bp| Blueprints::from_input(bp))
        .collect::<Vec<_>>();

    let mut p1 = 0;
    for blueprint in blueprints.iter_mut() {
        let quality_level = blueprint.id * blueprint.find_largest_number_of_geode();
        // println!("{quality_level}");
        p1 += quality_level
    }

    println!("{p1}");
}
