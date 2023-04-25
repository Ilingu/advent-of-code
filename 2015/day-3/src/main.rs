use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    // p1
    {
        let (mut x, mut y) = (0, 0);
        let mut visited = vec![(0, 0)];
        for dir in input.chars() {
            match dir {
                '>' => x += 1,
                '<' => x -= 1,
                '^' => y += 1,
                'v' => y -= 1,
                _ => panic!(),
            }
            if !visited.contains(&(x, y)) {
                visited.push((x, y));
            }
        }

        let p1 = visited.len();
        println!("{p1}")
    }

    // p2
    {
        let (mut x_santa, mut y_santa) = (0, 0);
        let (mut x_robot, mut y_robot) = (0, 0);
        let (mut x, mut y) = (0, 0);

        let mut visited = vec![(0, 0)];
        for (i, dir) in input.chars().enumerate() {
            match dir {
                '>' => x += 1,
                '<' => x -= 1,
                '^' => y += 1,
                'v' => y -= 1,
                _ => panic!(),
            }
            if !visited.contains(&(x, y)) {
                visited.push((x, y));
            }

            if i % 2 == 0 {
                (x_santa, y_santa) = (x, y);
                (x, y) = (x_robot, y_robot)
            } else {
                (x_robot, y_robot) = (x, y);
                (x, y) = (x_santa, y_santa)
            }
        }

        let p2 = visited.len();
        println!("{p2}")
    }
}
