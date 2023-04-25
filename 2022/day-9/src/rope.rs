use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Rope {
    body: Vec<Point>,
    tail_history: HashMap<(i32, i32), bool>,
}

impl Rope {
    pub fn new(body_len: usize) -> Self {
        Self {
            body: vec![Point { x: 0, y: 0 }; body_len],
            tail_history: HashMap::new(),
        }
    }

    pub fn head(&mut self) -> &mut Point {
        self.body.first_mut().unwrap()
    }
    pub fn tail(&mut self) -> &mut Point {
        self.body.last_mut().unwrap()
    }

    /// moves the head and updates the tail pos
    pub fn move_head(&mut self, (direction, amount): (&str, usize)) {
        for _ in 0..amount {
            match direction {
                "U" => self.head().y += 1,
                "D" => self.head().y -= 1,
                "R" => self.head().x += 1,
                "L" => self.head().x -= 1,
                _ => panic!("invalid direction"),
            }

            self.update_tail_pos();
        }
    }

    pub fn tail_history_len(&self) -> usize {
        return self.tail_history.len();
    }

    fn update_tail_pos(&mut self) {
        for c_id in 0..self.body.len() {
            if c_id + 1 > self.body.len() - 1 {
                break;
            }

            let (h, t) = (self.body[c_id], self.body.get_mut(c_id + 1).unwrap());

            let (delta_x, delta_y) = (h.x - t.x, h.y - t.y);
            let (is_left, is_below) = (delta_x.is_positive(), delta_y.is_positive());
            let (is_far_x, is_far_y) = (delta_x.abs() > 1, delta_y.abs() > 1);

            let mut update_x = || match is_left {
                true => t.x += 1,
                false => t.x -= 1,
            };
            let mut update_y = || match is_below {
                true => t.y += 1,
                false => t.y -= 1,
            };

            if is_far_x && delta_y == 0 {
                update_x();
            } else if is_far_y && delta_x == 0 {
                update_y();
            } else if is_far_x || is_far_y {
                update_x();
                update_y();
            }

            let tail = self.tail().clone();
            self.tail_history.insert((tail.x, tail.y), true);
        }
    }
}
