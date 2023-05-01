use std::fs;

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    let mut grid_p1 = vec![vec![false; 1000]; 1000];
    let mut grid_p2: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    for instruction in input.lines() {
        let positions = instruction
            .split_whitespace()
            .filter(|&s| s.contains(","))
            .map(|pos| {
                let position = pos
                    .split(",")
                    .map(|coord| coord.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (position[0], position[1])
            })
            .collect::<Vec<_>>();

        let (from, to) = (positions[0], positions[1]);
        let action = if instruction.contains("turn on") {
            Action::TurnOn
        } else if instruction.contains("turn off") {
            Action::TurnOff
        } else {
            Action::Toggle
        };

        for y in (from.1)..=(to.1) {
            for x in (from.0)..=(to.0) {
                match action {
                    Action::TurnOn => {
                        grid_p1[y][x] = true;
                        grid_p2[y][x] += 1;
                    }
                    Action::TurnOff => {
                        grid_p1[y][x] = false;
                        grid_p2[y][x] = (grid_p2[y][x] as isize - 1).max(0) as usize;
                    }
                    Action::Toggle => {
                        grid_p1[y][x] = !grid_p1[y][x];
                        grid_p2[y][x] += 2;
                    }
                }
            }
        }
    }

    let p1 = grid_p1
        .iter()
        .map(|x| {
            x.iter()
                .fold(0, |acc, &pos| if pos { acc + 1 } else { acc })
        })
        .sum::<i32>();
    println!("{p1}");
    let p2 = grid_p2
        .iter()
        .map(|x| x.iter().sum::<usize>())
        .sum::<usize>();
    println!("{p2}");
}
