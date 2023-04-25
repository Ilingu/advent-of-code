#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn from_string(string: String) -> Self {
        let parsed = string.split(";").collect::<Vec<&str>>();
        return Self {
            x: parsed[0].parse::<isize>().unwrap(),
            y: parsed[1].parse::<isize>().unwrap(),
        };
    }

    pub fn as_string(&self) -> String {
        return format!("{};{}", self.x, self.y);
    }

    pub fn manhattan_distance(Pos { x: x1, y: y1 }: &Pos, Pos { x: x2, y: y2 }: &Pos) -> usize {
        return ((x1 - x2).abs() + (y1 - y2).abs()) as usize;
    }
}
