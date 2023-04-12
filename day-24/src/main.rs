use std::{collections::HashMap, fs};

type Pos = (isize, isize);

fn main() {
    /* Parsing input */
    let input = fs::read_to_string("./Assets/input.txt").unwrap();

    let mut original_grid: HashMap<Pos, Vec<char>> = HashMap::new();
    let (height, width) = (
        input.lines().count() as isize,
        input.lines().nth(0).unwrap().chars().count() as isize,
    );

    for (raw_id, raw) in input.lines().enumerate() {
        for (col_id, entity) in raw.chars().enumerate() {
            if entity == '.' || entity == '#' {
                continue;
            }

            original_grid.insert((col_id as isize, raw_id as isize), vec![entity]);
        }
    }

    /* Search Period and corresponding states */
    let mut grid_overtime = original_grid.clone();
    let mut grid_snapshots = vec![original_grid.clone()];

    loop {
        let mut updated_pos: Vec<(Pos, char)> = vec![];
        for (&(x, y), entities) in &grid_overtime {
            for &entity in entities {
                let mut new_pos;
                match entity {
                    '^' => {
                        new_pos = (x, y - 1);
                        if new_pos.1 <= 0 {
                            new_pos.1 = height - 2
                        }
                    }
                    'v' => {
                        new_pos = (x, y + 1);
                        if new_pos.1 >= height - 1 {
                            new_pos.1 = 1
                        }
                    }
                    '>' => {
                        new_pos = (x + 1, y);
                        if new_pos.0 >= width - 1 {
                            new_pos.0 = 1
                        }
                    }
                    '<' => {
                        new_pos = (x - 1, y);
                        if new_pos.0 <= 0 {
                            new_pos.0 = width - 2
                        }
                    }
                    _ => panic!(),
                }
                updated_pos.push((new_pos, entity));
            }
        }

        grid_overtime.clear();
        for (new_pos, entity) in updated_pos.clone() {
            grid_overtime
                .entry(new_pos)
                .and_modify(|entities| entities.push(entity))
                .or_insert(vec![entity]);
        }

        if grid_overtime == original_grid {
            break;
        }
        grid_snapshots.push(grid_overtime.clone());
    }

    /* BFS - search shortest path */
    let (start, end) = ((1, 0), (width - 2, height - 1));
    let p1 = travel_valley(start, end, (width, height), grid_snapshots.clone(), 0);
    println!("{p1}");

    let go_get_snack = travel_valley(end, start, (width, height), grid_snapshots.clone(), p1);
    let go_back_to_end = travel_valley(
        start,
        end,
        (width, height),
        grid_snapshots.clone(),
        p1 + go_get_snack,
    );

    let p2 = p1 + go_get_snack + go_back_to_end;
    println!("{p2}");
}

fn travel_valley(
    start: (isize, isize),
    end: (isize, isize),
    (width, height): (isize, isize),
    grid_snapshots: Vec<HashMap<(isize, isize), Vec<char>>>,
    offset: usize,
) -> usize {
    let period = grid_snapshots.len();

    let mut queue = vec![(start.0, start.1, 1 as usize)];
    return loop {
        let (cx, cy, time) = queue.remove(0);
        let pos = (cx, cy);

        let current_grid = &grid_snapshots[(time + offset).rem_euclid(period)];
        let mut neighbourg = get_neighbourg(pos)
            .iter()
            .filter(|&&(x, y)| {
                let is_storm = current_grid.get(&(x, y)).is_some();
                return ((x, y) == end)
                    || (!is_storm && x > 0 && y > 0 && x <= width - 2 && y <= height - 2);
            })
            .map(|&p| (p.0, p.1, time + 1))
            .collect::<Vec<_>>();
        if current_grid.get(&pos).is_none() {
            neighbourg.push((cx, cy, time + 1));
        }

        if neighbourg.contains(&(end.0, end.1, time + 1)) {
            break time;
        }

        neighbourg = neighbourg
            .iter()
            .filter(|st| !queue.contains(st))
            .map(|&x| x)
            .collect();

        neighbourg
            .sort_by(|&a, &b| manhattan_distance(tp(a), end).cmp(&manhattan_distance(tp(b), end)));

        queue.append(&mut neighbourg);
    };
}

fn get_neighbourg((x, y): Pos) -> [Pos; 4] {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

fn tp(p: (isize, isize, usize)) -> (isize, isize) {
    (p.0, p.1)
}

fn manhattan_distance((x1, y1): Pos, (x2, y2): Pos) -> usize {
    ((x1 - x2).abs() + (y1 - y2).abs()) as usize
}
