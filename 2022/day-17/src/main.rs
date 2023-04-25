pub mod cave;

use std::fs;

use cave::Cave;

fn main() {
    let jets = fs::read_to_string("./Assets/input.txt")
        .unwrap()
        .chars()
        .map(|c| c.to_string())
        .collect();

    let mut cave = Cave::default();
    let datas = cave.simulate_falling_rocks(&jets, 20_000);

    println!("{}", cave.get_max_height()); // p1

    let (period, commom_ratio, cycle_rock_id, curr_height) = find_cycle(datas);

    println!(
        "{}",
        ((1e12 - cycle_rock_id as f64) * commom_ratio as f64 / period as f64) + curr_height as f64
    ) // p2
}

fn find_cycle(datas: Vec<(usize, usize)>) -> (usize, usize, usize, usize) {
    for (i, outer_data) in datas.iter().enumerate() {
        for (fake_j, inner_data) in datas[i + 1..].iter().enumerate() {
            let j = i + 1 + fake_j;
            let (delta_id, delta_value) =
                (inner_data.0 - outer_data.0, inner_data.1 - outer_data.1);

            let mut future_idx = j + delta_id;
            let mut last_future_value = inner_data.1;
            let mut succeed_count = 0;
            loop {
                if succeed_count >= 5 {
                    return (delta_id, delta_value, outer_data.0 + 1, outer_data.1);
                }
                if future_idx > datas.len() - 1 {
                    break;
                }

                let (_, future_value) = datas[future_idx];
                if future_value - last_future_value == delta_value {
                    future_idx += delta_id;
                    succeed_count += 1;
                    last_future_value = future_value;
                    continue;
                } else {
                    break;
                }
            }
        }
    }
    (0, 0, 0, 0)
}
