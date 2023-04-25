use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
enum CaveEntity {
    FallingRock,
    StoppedRock,
}

pub struct Cave {
    grid: HashMap<Pos, CaveEntity>,
}

impl Cave {
    pub fn default() -> Self {
        Self {
            grid: HashMap::new(),
        }
    }

    pub fn get_max_height(&self) -> usize {
        let mut heighest = 0;
        for Pos { x: _, y } in self.grid.keys() {
            if y + 1 > heighest {
                heighest = y + 1
            }
        }
        heighest
    }

    pub fn simulate_falling_rocks(
        &mut self,
        jet_sequence: &Vec<String>,
        stop_rock_id: usize,
    ) -> Vec<(usize, usize)> {
        let (mut tick_id, mut rock_id) = (0, 0);
        let mut current_falling_rock = self.inject_new_rock(rock_id);

        let mut logs_rockid_maxheight: Vec<(usize, usize)> = vec![];
        loop {
            let gas_jet = jet_sequence[tick_id % jet_sequence.len()].as_str();

            let (mut new_positions, mut is_ok): (Vec<Pos>, bool) = (vec![], true);
            for Pos { x, y } in &current_falling_rock {
                if (x >= &6 && gas_jet == ">") || (x <= &0 && gas_jet == "<") {
                    is_ok = false;
                    break;
                }

                let new_side_pos = Pos::new(if gas_jet == ">" { x + 1 } else { x - 1 }, *y);
                match self.grid.get(&new_side_pos) {
                    Some(entity) => match entity {
                        CaveEntity::StoppedRock => {
                            is_ok = false;
                            break;
                        }
                        _ => {}
                    },
                    None => {}
                }

                new_positions.push(new_side_pos)
            }

            // move left/right
            if is_ok {
                for old_pos in &current_falling_rock {
                    self.grid.remove(old_pos);
                }
                for np in new_positions.clone() {
                    self.grid.insert(np, CaveEntity::FallingRock);
                }
                current_falling_rock = new_positions;
            }
            (new_positions, is_ok) = (vec![], true);

            // move down
            for Pos { x, y } in &current_falling_rock {
                if y <= &0 {
                    is_ok = false;
                    break;
                }

                let new_down_pos = Pos::new(*x, y - 1);
                match self.grid.get(&new_down_pos) {
                    Some(entity) => match entity {
                        CaveEntity::StoppedRock => {
                            is_ok = false;
                            break;
                        }
                        _ => {}
                    },
                    None => {}
                }

                new_positions.push(new_down_pos)
            }

            if is_ok {
                for old_pos in &current_falling_rock {
                    self.grid.remove(old_pos);
                }
                for np in new_positions.clone() {
                    self.grid.insert(np, CaveEntity::FallingRock);
                }
                current_falling_rock = new_positions;
            } else {
                for old_pos in &current_falling_rock {
                    *self.grid.get_mut(old_pos).unwrap() = CaveEntity::StoppedRock;
                }

                logs_rockid_maxheight.push((rock_id, self.get_max_height()));
                rock_id += 1;
                if rock_id == stop_rock_id - 1 {
                    break;
                }

                current_falling_rock = self.inject_new_rock(rock_id);
            }

            tick_id += 1;
        }
        logs_rockid_maxheight
    }

    fn inject_new_rock(&mut self, rock_id: usize) -> Vec<Pos> {
        let rocks = vec![
            // "-"
            vec![
                Pos::new(2, 0),
                Pos::new(3, 0),
                Pos::new(4, 0),
                Pos::new(5, 0),
            ],
            // +
            vec![
                Pos::new(3, 0),
                Pos::new(2, 1),
                Pos::new(3, 1),
                Pos::new(4, 1),
                Pos::new(3, 2),
            ],
            //  inversed "L"
            vec![
                Pos::new(2, 0),
                Pos::new(3, 0),
                Pos::new(4, 0),
                Pos::new(4, 1),
                Pos::new(4, 2),
            ],
            // "I"
            vec![
                Pos::new(2, 0),
                Pos::new(2, 1),
                Pos::new(2, 2),
                Pos::new(2, 3),
            ],
            vec![
                Pos::new(2, 0),
                Pos::new(3, 0),
                Pos::new(2, 1),
                Pos::new(3, 1),
            ],
        ];

        let spawn_height = self.get_max_height() + 3;
        let rock_to_spawn = &rocks[rock_id % rocks.len()];

        let mut rock_pos: Vec<Pos> = vec![];
        for Pos { x, y } in rock_to_spawn {
            let point_pos = Pos::new(*x, y + spawn_height);
            rock_pos.push(point_pos);
            self.grid.insert(point_pos, CaveEntity::FallingRock);
        }
        rock_pos
    }
}
