use std::{collections::HashMap, fs};

type Pos = (isize, isize, isize);

#[derive(Debug)]
pub struct Cube {
    x: isize,
    y: isize,
    z: isize,
    exposed_side: u8,
}

impl Cube {
    pub fn from_tuple((x, y, z): Pos) -> Self {
        Self {
            x,
            y,
            z,
            exposed_side: 6,
        }
    }
    pub fn to_tuple(&self) -> Pos {
        (self.x, self.y, self.z)
    }
}

fn main() {
    let coordinates = fs::read_to_string("./Assets/baseinput.txt")
        .unwrap()
        .split("\n")
        .map(|sp| {
            sp.split(",")
                .map(|cstr| cstr.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut map: HashMap<Pos, bool> = HashMap::new();
    let mut cubes: Vec<Cube> = vec![];

    let (mut max_x, mut min_x) = (0, 0);
    let (mut max_y, mut min_y) = (0, 0);
    let (mut max_z, mut min_z) = (0, 0);

    for coordinate in coordinates {
        let xyz = (coordinate[0], coordinate[1], coordinate[2]);

        {
            // max/min x
            max_x = xyz.0.max(max_x);
            min_x = xyz.0.min(min_x);

            // max/min y
            max_y = xyz.1.max(max_y);
            min_y = xyz.1.min(min_y);

            // max/min z
            max_z = xyz.2.max(max_z);
            min_z = xyz.2.min(min_z);
        }

        cubes.push(Cube::from_tuple(xyz));
        map.insert(xyz, true);
    }

    let mut air_coordinates: Vec<Pos> = vec![];
    for Cube {
        x,
        y,
        z,
        exposed_side,
    } in cubes.iter_mut()
    {
        let abjacent_pos = get_abjacent_pos((*x, *y, *z));
        for abj_coord in abjacent_pos {
            match map.get(&abj_coord) {
                Some(_) => *exposed_side = *exposed_side - 1,
                None => air_coordinates.push(abj_coord),
            }
        }
    }

    let p1 = cubes.iter().map(|x| x.exposed_side as usize).sum::<usize>();
    println!("{p1}");

    let mut air_sides_counts: HashMap<Pos, usize> = HashMap::new();
    for air_coord in air_coordinates.clone() {
        air_sides_counts
            .entry(air_coord)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut faces_to_extract: usize = 0;
    for air_coord in air_coordinates {
        match is_inside_rock(&map, (max_x, min_x, max_y, min_y, max_z, min_z), air_coord) {
            true => faces_to_extract += air_sides_counts.get(&air_coord).unwrap(),
            false => {}
        }
    }

    let p2 = p1 - faces_to_extract;
    println!("{p2}");
}

fn get_abjacent_pos((x, y, z): Pos) -> [Pos; 6] {
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

fn is_inside_rock(
    map: &HashMap<Pos, bool>,
    (max_x, min_x, max_y, min_y, max_z, min_z): (isize, isize, isize, isize, isize, isize),
    air_coord: Pos,
) -> bool {
    let mut abjacent_pos: Vec<Pos> = get_abjacent_pos(air_coord).to_vec();
    let mut visited: Vec<Pos> = vec![air_coord];

    while abjacent_pos.len() != 0 {
        for abj_pos in &abjacent_pos {
            match map.get(abj_pos) {
                Some(_) => {}
                None => {}
            }
        }
    }
    true
}
