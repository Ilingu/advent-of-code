use std::fs;

fn main() {
    let raw_file = fs::read_to_string("./Assets/input.txt").unwrap();

    // P1
    {
        let original_list = raw_file
            .lines()
            .enumerate()
            .map(|(id, val)| (id, val.parse::<isize>().unwrap()))
            .collect::<Vec<_>>();

        let decrypted_list = mix(&original_list, &original_list);
        let p1 = compute_coord(decrypted_list);
        println!("{p1}");
    }

    // P2
    {
        const DECRYPTION_KEY: isize = 811589153;
        let original_order = raw_file
            .lines()
            .enumerate()
            .map(|(id, val)| (id, val.parse::<isize>().unwrap() * DECRYPTION_KEY))
            .collect::<Vec<_>>();

        let mut decrypted_list = original_order.clone();
        for _ in 0..10 {
            decrypted_list = mix(&original_order, &decrypted_list);
        }

        let p2 = compute_coord(decrypted_list);
        println!("{p2}");
    }
}

fn mix(
    original_order: &Vec<(usize, isize)>,
    original_list: &Vec<(usize, isize)>,
) -> Vec<(usize, isize)> {
    let len = original_list.len() as isize;
    let mut decypted_order = original_list.clone();

    for (id, num) in original_order {
        let curr_idx = decypted_order
            .iter()
            .position(|(curr_id, _)| curr_id == id)
            .unwrap();
        let new_idx = (curr_idx as isize + num).rem_euclid(len - 1);

        decypted_order.remove(curr_idx);
        decypted_order.insert(new_idx as usize, (*id, *num));
    }
    decypted_order
}

fn compute_coord(decrypted_list: Vec<(usize, isize)>) -> isize {
    let zero_pos = decrypted_list.iter().position(|(_, x)| x == &0).unwrap();

    let (x, y, z) = (
        decrypted_list[(zero_pos + (1000 % decrypted_list.len())) % decrypted_list.len()].1,
        decrypted_list[(zero_pos + (2000 % decrypted_list.len())) % decrypted_list.len()].1,
        decrypted_list[(zero_pos + (3000 % decrypted_list.len())) % decrypted_list.len()].1,
    );
    x + y + z
}
