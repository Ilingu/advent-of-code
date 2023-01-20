pub mod graph;

use std::fs;

use graph::Graph;

fn main() {
    let valves_report = fs::read_to_string("./Assets/baseinput.txt").unwrap();
    let graph = Graph::parse(&valves_report);

    // graph.get_best_pressure_score_by_brute_force();

    let score = graph.get_best_pressure_score();
    println!("{score}");
}
