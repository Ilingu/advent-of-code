use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn parse(str_to_parse: &str) -> Self {
        let raw_coord = str_to_parse.split(",").collect::<Vec<&str>>();
        assert_eq!(raw_coord.len(), 2);

        return Self {
            x: raw_coord[0].parse::<usize>().unwrap(),
            y: raw_coord[1].parse::<usize>().unwrap(),
        };
    }

    pub fn to_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn to_string(&self) -> String {
        return format!("{},{}", self.x, self.y);
    }
}

#[derive(PartialEq)]
enum CaveElements {
    AIR,
    OBSTACLE,
    VOID,
}

#[derive(Debug)]
pub struct Cave {
    obstacles_pos: HashSet<String>, // string should be under this format: "x,y"
    sand_rest_pos: HashSet<String>,
    ext_y: (usize, usize), // (min_y=0, max_y)
    ext_x: (usize, usize), // (min_x, max_y)

    pub is_part_2: bool,
}

impl Cave {
    pub fn build_cave(scans: &str) -> Self {
        let mut max_y = 0;
        let (mut min_x, mut max_x) = (500, 500);
        let mut obstacles_pos: HashSet<String> = HashSet::new();

        for scan in scans.lines() {
            let mut last_cood: Option<Pos> = None;

            let points = scan.split(" -> ");
            for point in points {
                let coordinates = Pos::parse(point);
                if coordinates.y > max_y {
                    max_y = coordinates.y;
                }
                if coordinates.x > max_x {
                    max_x = coordinates.x;
                } else if coordinates.x < min_x {
                    min_x = coordinates.x;
                }

                match last_cood {
                    Some(Pos {
                        x: x_last,
                        y: y_last,
                    }) => match coordinates.x == x_last {
                        true => {
                            let y_range = if y_last <= coordinates.y {
                                y_last..=coordinates.y
                            } else {
                                (coordinates.y)..=y_last
                            };
                            for y in y_range {
                                obstacles_pos.insert(Pos::new(coordinates.x, y).to_string());
                            }
                        }
                        false => {
                            let x_range = if x_last <= coordinates.x {
                                x_last..=coordinates.x
                            } else {
                                (coordinates.x)..=x_last
                            };
                            for x in x_range {
                                obstacles_pos.insert(Pos::new(x, coordinates.y).to_string());
                            }
                        }
                    },
                    None => {
                        obstacles_pos.insert(coordinates.to_string());
                    }
                }

                last_cood = Some(coordinates);
            }
        }

        return Self {
            obstacles_pos,
            sand_rest_pos: HashSet::new(),
            ext_y: (0, max_y),
            ext_x: (min_x, max_x),
            is_part_2: false,
        };
    }

    pub fn clear_sand(&mut self) {
        self.sand_rest_pos = HashSet::new();
    }

    pub fn fill_cave(&mut self) -> usize {
        self.clear_sand();

        let mut sand_units_dropped: usize = 0;
        while !self.drop_sand() {
            sand_units_dropped += 1;
        }

        return sand_units_dropped;
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand_pos = Pos::new(500, 0);

        loop {
            let sand_next_pos = Pos::new(sand_pos.x, sand_pos.y + 1);
            match self.pos_element(&sand_next_pos) {
                CaveElements::AIR => sand_pos = sand_next_pos,
                CaveElements::OBSTACLE => {
                    let left_pos = Pos::new(sand_next_pos.x - 1, sand_next_pos.y);
                    if self.pos_element(&left_pos) != CaveElements::OBSTACLE {
                        sand_pos = left_pos;
                        continue;
                    }

                    let right_pos = Pos::new(sand_next_pos.x + 1, sand_next_pos.y);
                    if self.pos_element(&right_pos) == CaveElements::OBSTACLE {
                        if self.is_part_2 && sand_pos == Pos::new(500, 0) {
                            return true;
                        }
                        self.sand_rest_pos.insert(sand_pos.to_string());
                        break;
                    }
                    sand_pos = right_pos;
                }
                CaveElements::VOID => return true,
            }
        }
        return false;
    }

    fn pos_element(&self, pos: &Pos) -> CaveElements {
        let is_obstacle = self.obstacles_pos.contains(&pos.to_string())
            || self.sand_rest_pos.contains(&pos.to_string());
        if is_obstacle || (self.is_part_2 && pos.y == self.ext_y.1 + 2) {
            return CaveElements::OBSTACLE;
        }

        return if (pos.x < self.ext_x.0 || pos.x > self.ext_x.1 || pos.y > self.ext_y.1)
            && !self.is_part_2
        {
            CaveElements::VOID
        } else {
            CaveElements::AIR
        };
    }
}
