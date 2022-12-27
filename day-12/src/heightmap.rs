use std::{collections::HashMap, vec};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Clone)]
enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn from_tuple((x, y): (usize, usize)) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        return (self.x as usize, self.y as usize);
    }
}

#[derive(Debug)]
pub struct HeightMap {
    width: usize,
    height: usize,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
    grid: HashMap<(usize, usize), char>, // (col, row)
}

impl HeightMap {
    pub fn parse(input: &str) -> Self {
        let mut grid = HashMap::new();
        let height: usize = input.lines().count();

        let (mut start_pos, mut end_pos) = ((0, 0), (0, 0));
        for (row_id, row) in input.lines().enumerate() {
            for (col_id, ch) in row.chars().enumerate() {
                grid.insert((col_id, row_id), ch);
                match ch {
                    'S' => start_pos = (col_id, row_id),
                    'E' => end_pos = (col_id, row_id),
                    _ => {}
                }
            }
        }

        return Self {
            width: grid.len() / height,
            height,
            start_pos,
            end_pos,
            grid,
        };
    }

    pub fn shortest_path(&self) -> usize {
        self.bfs(self.start_pos).unwrap()
    }

    pub fn shortest_a_path(&self) -> usize {
        let mut shortest_paths: Vec<usize> = vec![];
        for (pos, ch) in self.grid.iter() {
            if ch == &'a' || ch == &'S' {
                match self.bfs(*pos) {
                    Some(sortest) => shortest_paths.push(sortest),
                    None => {}
                }
            }
        }
        return *shortest_paths.iter().min().unwrap();
    }

    fn bfs(&self, entry: (usize, usize)) -> Option<usize> {
        let mut queue: Vec<Pos> = vec![Pos::from_tuple(entry)];
        let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
        distances.insert(entry, 0);

        while queue.len() != 0 {
            let node = queue.remove(0);
            for (next_pos, _) in self.possible_next_pos(node) {
                if distances.get(&next_pos.as_tuple()) == None {
                    queue.push(next_pos);
                    distances.insert(
                        next_pos.as_tuple(),
                        distances.get(&node.as_tuple()).unwrap() + 1,
                    );
                }
            }
        }
        return distances.get(&self.end_pos).cloned();
    }

    fn cmp_height(a: &char, b: &char) -> i32 {
        let parse_unresolved = |x: char| match x {
            'S' => 'a',
            'E' => 'z',
            _ => x,
        };

        let a_index = ALPHABET.find(&parse_unresolved(*a).to_string()).unwrap() as i32;
        let b_index = ALPHABET.find(&parse_unresolved(*b).to_string()).unwrap() as i32;
        return b_index - a_index;
    }
    fn possible_next_pos(&self, pos: Pos) -> Vec<(Pos, Directions)> {
        let curr_height = self.grid.get(&pos.as_tuple()).unwrap();

        let (x, y) = (pos.x, pos.y);
        let surrondings: [(Pos, Directions); 4] = [
            (Pos { x, y: y - 1 }, Directions::UP),    // up neighbour
            (Pos { x, y: y + 1 }, Directions::DOWN),  // down neighbour
            (Pos { x: x - 1, y }, Directions::LEFT),  // left neighbour
            (Pos { x: x + 1, y }, Directions::RIGHT), // right neighbour
        ];

        let (grid_h, grid_w) = (self.height as isize, self.width as isize);

        let possible_dir = surrondings
            .iter()
            .filter(|(pos, _)| pos.x >= 0 && pos.y >= 0 && pos.x < grid_w && pos.y < grid_h)
            .filter(|(pos, _)| {
                let pos_height = self.grid.get(&pos.as_tuple()).unwrap();
                return Self::cmp_height(curr_height, pos_height) <= 1;
            })
            .cloned()
            .collect::<Vec<(Pos, Directions)>>();

        return possible_dir;
    }
}
