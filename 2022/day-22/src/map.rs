use std::collections::HashMap;

use crate::Instruction;

type Pos = (usize, usize); // (x,y)

#[derive(Debug)]
pub enum Entity {
    Tile,
    Wall,
}

#[derive(Debug)]
pub struct Map {
    pub current_tile: Pos,
    pub orientation: isize,
    starting_pos: Pos,
    grid: HashMap<Pos, Entity>,
    raw_extremum: HashMap<usize, (Pos, Pos)>, // raw_id: (leftmost_pos, rightmost_pos)
    col_extremum: HashMap<usize, (Pos, Pos)>, // col_id: (topmost_pos, bottommost_pos)
}

impl Map {
    pub fn load(map_input: &str) -> Self {
        let mut starting_pos: Option<Pos> = None;
        let mut grid = HashMap::new();
        let mut raw_extremum = HashMap::new();
        let (mut col_top, mut col_bottom) = (HashMap::new(), HashMap::new());

        for (raw_id, raw) in map_input.lines().enumerate() {
            let (mut first_tile, mut last_tile): (Option<Pos>, Pos) = (None, (0, 0));
            for (col_id, entity) in raw.chars().enumerate() {
                if entity == ' ' {
                    continue;
                }

                let pos: Pos = (col_id + 1, raw_id + 1);
                if col_top.get(&col_id).is_none() {
                    col_top.insert(col_id, pos);
                }
                col_bottom.insert(col_id, pos);

                if first_tile.is_none() {
                    first_tile = Some(pos);
                    if starting_pos.is_none() {
                        starting_pos = Some(pos)
                    }
                }
                last_tile = pos;

                match entity {
                    '.' => grid.insert(pos, Entity::Tile),
                    '#' => grid.insert(pos, Entity::Wall),
                    _ => panic!("shouldn't have happen"),
                };
            }
            raw_extremum.insert(raw_id + 1, (first_tile.unwrap(), last_tile));
        }

        let mut col_extremum = HashMap::new();
        for (col_id, top_pos) in col_top {
            let bot_pos = col_bottom[&col_id];

            col_extremum.insert(col_id + 1, (top_pos, bot_pos));
        }

        Self {
            current_tile: starting_pos.unwrap(),
            orientation: 0,
            starting_pos: starting_pos.unwrap(),
            grid,
            col_extremum,
            raw_extremum,
        }
    }

    pub fn reset(&mut self) {
        self.current_tile = self.starting_pos;
        self.orientation = 0;
    }

    pub fn convert_orientation(current_orientation: isize) -> usize {
        match current_orientation {
            0 => 0,
            90 => 3,
            180 => 2,
            270 => 1,
            _ => panic!("not intented"),
        }
    }

    pub fn execute_instructions(&mut self, instructions: &Vec<Instruction>) {
        for &Instruction {
            orientation_to_apply,
            move_tile,
        } in instructions
        {
            self.orientation = (self.orientation + orientation_to_apply as isize).rem_euclid(360);
            self.move_cursor(move_tile);
        }
    }

    fn move_cursor(&mut self, number_of_tile: usize) {
        for _ in 0..number_of_tile {
            let next_pos = self.get_next_tile();

            match self.grid.get(&next_pos) {
                Some(entity) => match entity {
                    Entity::Tile => self.current_tile = next_pos,
                    Entity::Wall => return,
                },
                None => panic!("not intended"),
            }
        }
    }

    fn get_next_tile(&mut self) -> Pos {
        let (x, y) = self.current_tile;
        match self.orientation {
            0 => {
                let (leftmost, rightmost) = self.raw_extremum[&y];
                if x == rightmost.0 {
                    return leftmost;
                }
                (x + 1, y)
            } // right
            90 => {
                let (topmost, botmost) = self.col_extremum[&x];
                if y == topmost.1 {
                    return botmost;
                }
                (x, y - 1)
            } // top
            180 => {
                let (leftmost, rightmost) = self.raw_extremum[&y];
                if x == leftmost.0 {
                    return rightmost;
                }
                (x - 1, y)
            } // left
            270 => {
                let (topmost, botmost) = self.col_extremum[&x];
                if y == botmost.1 {
                    return topmost;
                }
                (x, y + 1)
            } // bottom
            _ => panic!("not intended"),
        }
    }

    
}
