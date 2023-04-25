use std::{fmt::Debug, fs, str::FromStr, vec};

fn main() {
    let input = fs::read_to_string("./src/Assets/input.txt").unwrap();

    // Parsing input
    let input_parts = Vec::from_iter(input.split("\n\n"));
    if input_parts.len() != 2 {
        panic!("input_parts.len() != 2")
    }

    let crates_template = input_parts[0];
    let instructions = input_parts[1];

    let mut crates: Vec<Vec<u8>> = vec![];
    parse_crates(crates_template, &mut crates);

    let mut crates_part_2 = crates.clone();

    // println!("{:#?}", crates_part_2);

    for instruction in instructions.lines() {
        let datas: Vec<&str> = instruction.split_whitespace().collect();
        if datas.len() != 6 {
            panic!("datas.len() != 6")
        }

        let (nb_of_moves, from_old_crate_id, to_new_create_id) = (
            to_number::<usize>(datas[1]),
            to_number::<usize>(datas[3]) - 1,
            to_number::<usize>(datas[5]) - 1,
        );

        // P1
        for _ in 0..nb_of_moves {
            let elem_to_move = crates[from_old_crate_id].first().unwrap().clone();
            crates[from_old_crate_id].remove(0);
            crates[to_new_create_id].insert(0, elem_to_move);
        }

        // P2
        let mut elems_to_move: Vec<u8> = crates_part_2[from_old_crate_id][0..nb_of_moves]
            .iter()
            .clone()
            .map(|x| *x)
            .collect();

        elems_to_move.append(&mut crates_part_2[to_new_create_id]);
        crates_part_2[to_new_create_id] = elems_to_move;

        crates_part_2[from_old_crate_id].drain(0..nb_of_moves);
    }

    let mut p1 = String::new();
    for final_crate in crates {
        let top_stack_char = [final_crate.first().unwrap().clone()];
        let top_stack = std::str::from_utf8(&top_stack_char).unwrap();
        p1 += top_stack;
    }

    let mut p2 = String::new();
    for final_crate in crates_part_2 {
        let top_stack_char = [final_crate.first().unwrap().clone()];
        let top_stack = std::str::from_utf8(&top_stack_char).unwrap();
        p2 += top_stack;
    }

    println!("{p1}");
    println!("{p2}")
}

// const COL_ID: [usize; 9] = [1, 5, 9, 13, 17, 21, 25, 29, 33]; --> old way of having cold_ids, it has hard coded
const WHITE_SPACE: u8 = 32;

fn parse_crates(crates_template: &str, crates: &mut Vec<Vec<u8>>) {
    let mut format_template = Vec::from_iter(crates_template.lines());

    // parsing col_ids to checks
    let col_ids = format_template
        .last()
        .unwrap()
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, c)| **c != WHITE_SPACE)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    // if col_ids.len() != 9 {
    //     panic!("col_ids.len() != 9")
    // }
    format_template.pop();

    *crates = vec![vec![]; col_ids.len()];

    for row_id in 0..format_template.len() {
        for crate_id in 0..col_ids.len() {
            let b = format_template[row_id].as_bytes()[col_ids[crate_id]];
            if b != WHITE_SPACE {
                crates[crate_id].push(b);
            }
        }
    }
}

fn to_number<T: std::str::FromStr>(str: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    str.parse::<T>().unwrap()
}
