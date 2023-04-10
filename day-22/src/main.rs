pub mod map;

use std::fs;

use map::Map;

#[derive(Debug)]
pub struct Instruction {
    orientation_to_apply: i8,
    move_tile: usize,
}

impl Instruction {
    pub fn generate_instruction_list(input_instruction: &str) -> Vec<Self> {
        let mut instruction_list = vec![];

        let mut last_instruction_idx = 0;

        let mut add_instruction = |instruction: &str| {
            let mut orientation_to_apply = 90;
            if instruction.contains("R") {
                orientation_to_apply = -90
            }

            let move_tile = instruction
                .trim_start_matches("R")
                .trim_start_matches("L")
                .parse::<usize>()
                .unwrap();

            instruction_list.push(Instruction {
                orientation_to_apply,
                move_tile,
            });
        };

        for (i, ch) in input_instruction.chars().enumerate() {
            if ch == 'R' || ch == 'L' {
                add_instruction(&input_instruction[last_instruction_idx..i]);
                last_instruction_idx = i;
            }
        }
        add_instruction(&input_instruction[last_instruction_idx..]);

        instruction_list[0].orientation_to_apply = 0;
        instruction_list
    }
}

fn main() {
    let raw_input = fs::read_to_string("./Assets/input.txt").unwrap();
    let binding = raw_input.split("\n\n").collect::<Vec<_>>();

    let (raw_map, raw_instructions) = (binding[0], binding[1]);

    let instructions_list = Instruction::generate_instruction_list(raw_instructions);
    let mut map = Map::load(raw_map);

    {
        map.execute_instructions(&instructions_list);
        let p1 = 1000 * map.current_tile.1
            + 4 * map.current_tile.0
            + Map::convert_orientation(map.orientation);
        println!("{p1}")
    }
}
