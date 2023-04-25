use std::{collections::HashMap, fs};

type Pos = (isize, isize);

fn main() {
    let input = fs::read_to_string("./Assets/input.txt").unwrap();

    let mut grid: HashMap<Pos, usize> = HashMap::new();
    // Parsing input
    {
        let mut elf_id: usize = 0;
        for (raw_id, raw) in input.lines().enumerate() {
            for (col_id, entity) in raw.chars().enumerate() {
                if entity == '#' {
                    grid.insert((col_id as isize, raw_id as isize), elf_id);
                    elf_id += 1
                }
            }
        }
    }

    let mut elf_direction_proposition = vec!['N', 'S', 'W', 'E'];
    let mut round_id: usize = 0;

    loop {
        if round_id == 10 {
            let (max_north, max_south, max_west, max_east) = fetch_extremum_elfs(&grid);
            let (length, widelength) = (
                (max_north - max_south).abs() + 1,
                (max_west - max_east).abs() + 1,
            );
            let area = length * widelength;

            let p1 = area - grid.len() as isize;
            println!("{p1}");
        }

        let mut elfs_propositions: Vec<(Pos, Pos, usize)> = vec![]; // (current_pos, next_pos)
        for (&(x, y), &elf_id) in &grid {
            let abjacent_pos = vec![
                (x, y - 1),
                (x - 1, y - 1),
                (x + 1, y - 1),
                (x, y + 1),
                (x - 1, y + 1),
                (x + 1, y + 1),
                (x - 1, y),
                (x + 1, y),
            ];
            if !are_there_elfs(&grid, abjacent_pos) {
                continue;
            }

            for direction in &elf_direction_proposition {
                match direction {
                    'N' => {
                        let to_check = vec![(x, y - 1), (x - 1, y - 1), (x + 1, y - 1)];
                        if !are_there_elfs(&grid, to_check) {
                            elfs_propositions.push(((x, y), (x, y - 1), elf_id));
                            break;
                        }
                    }
                    'S' => {
                        let to_check = vec![(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
                        if !are_there_elfs(&grid, to_check) {
                            elfs_propositions.push(((x, y), (x, y + 1), elf_id));
                            break;
                        }
                    }
                    'W' => {
                        let to_check = vec![(x - 1, y), (x - 1, y - 1), (x - 1, y + 1)];
                        if !are_there_elfs(&grid, to_check) {
                            elfs_propositions.push(((x, y), (x - 1, y), elf_id));
                            break;
                        }
                    }
                    'E' => {
                        let to_check = vec![(x + 1, y), (x + 1, y - 1), (x + 1, y + 1)];
                        if !are_there_elfs(&grid, to_check) {
                            elfs_propositions.push(((x, y), (x + 1, y), elf_id));
                            break;
                        }
                    }
                    _ => panic!(),
                }
            }
        }

        if elfs_propositions.len() == 0 {
            break;
        }

        let mut proposition_grouped = HashMap::new();
        for (_, new_pos, _) in elfs_propositions.clone() {
            proposition_grouped
                .entry(new_pos)
                .and_modify(|counter| *counter += 1 as usize)
                .or_insert(1);
        }

        for (current_pos, new_pos, elf_id) in elfs_propositions {
            if proposition_grouped[&new_pos] > 1 {
                continue;
            }

            grid.remove(&current_pos);
            grid.insert(new_pos, elf_id);
        }

        let first = elf_direction_proposition.remove(0);
        elf_direction_proposition.push(first);

        round_id += 1;
    }

    let p2 = round_id + 1;
    println!("{p2}");
}

fn fetch_extremum_elfs(grid: &HashMap<Pos, usize>) -> (isize, isize, isize, isize) {
    let (&(x_i, y_i), _) = grid.iter().nth(0).unwrap();
    let (mut max_north, mut max_south, mut max_west, mut max_east) = (y_i, y_i, x_i, x_i);

    for &(x, y) in grid.keys() {
        max_north = max_north.min(y);
        max_south = max_south.max(y);
        max_west = max_west.min(x);
        max_east = max_east.max(x);
    }
    (max_north, max_south, max_west, max_east)
}

fn are_there_elfs(grid: &HashMap<Pos, usize>, to_check: Vec<Pos>) -> bool {
    for pos in to_check {
        match grid.get(&pos) {
            Some(_) => return true,
            None => {}
        }
    }
    false
}
