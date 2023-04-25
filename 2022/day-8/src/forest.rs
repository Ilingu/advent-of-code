use std::{collections::HashMap, vec};

#[derive(Debug)]
pub struct Forest {
    heatmap: HashMap<String, u8>,
    max_cols: usize,
    max_rows: usize,
}

impl Forest {
    pub fn new(max_cols: usize, max_rows: usize) -> Self {
        Self {
            heatmap: HashMap::new(),
            max_cols,
            max_rows,
        }
    }

    pub fn from_input(inp: String) -> Self {
        let cols = inp.lines().collect::<Vec<&str>>();
        let mut map = Self::new(cols[0].len() - 1, cols.len() - 1);

        for (row_id, row) in cols.iter().enumerate() {
            for (col_id, ch) in row.chars().enumerate() {
                map.heatmap.insert(
                    format!("{col_id}-{row_id}"),
                    ch.to_string().parse::<u8>().unwrap(),
                );
            }
        }

        return map;
    }

    pub fn get_pos(&self, col_id: u8, row_id: u8) -> &u8 {
        return self
            .heatmap
            .get(&Self::generate_pos(col_id, row_id))
            .unwrap();
    }

    pub fn len(&self) -> usize {
        return self.heatmap.len();
    }

    fn generate_pos(col_id: u8, row_id: u8) -> String {
        return format!("{col_id}-{row_id}");
    }
    fn parse_pos(pos: &String) -> (u8, u8) {
        return match &pos.split("-").collect::<Vec<&str>>()[..] {
            &[col_id, row_id, ..] => (col_id.parse::<u8>().unwrap(), row_id.parse::<u8>().unwrap()),
            _ => unreachable!(),
        };
    }

    fn is_corner(forest: &Forest, (col_id, row_id): (u8, u8)) -> bool {
        let max_col = forest.max_cols as u8;
        let max_row = forest.max_rows as u8;
        return (col_id <= 0 || col_id >= max_col) || (row_id <= 0 || row_id >= max_row);
    }

    // P1
    pub fn count_visible(&self) -> usize {
        let mut count = 0;
        for (pos, height) in self.heatmap.iter() {
            let (col_id, row_id) = Self::parse_pos(pos);
            if Self::is_corner(self, (col_id, row_id)) {
                count += 1;
                continue;
            }

            let top = (0..row_id)
                .map(|x| (col_id, x))
                .filter(|(col_id, row_id)| self.get_pos(*col_id, *row_id) >= height)
                .count();
            let bottom = (row_id + 1..=self.max_rows.try_into().unwrap())
                .map(|x| (col_id, x))
                .filter(|(col_id, row_id)| self.get_pos(*col_id, *row_id) >= height)
                .count();
            let left = (0..col_id)
                .map(|x| (x, row_id))
                .filter(|(col_id, row_id)| self.get_pos(*col_id, *row_id) >= height)
                .count();
            let right = (col_id + 1..=self.max_cols.try_into().unwrap())
                .map(|x| (x, row_id))
                .filter(|(col_id, row_id)| self.get_pos(*col_id, *row_id) >= height)
                .count();

            if top <= 0 || bottom <= 0 || left <= 0 || right <= 0 {
                count += 1;
            }
        }

        return count;
    }

    // P2
    pub fn compute_scenic_score(&self) -> usize {
        let mut scenic_scores: Vec<usize> = vec![];

        for (pos, height) in self.heatmap.iter() {
            let (col_id, row_id) = Self::parse_pos(pos);
            if Self::is_corner(self, (col_id, row_id)) {
                continue;
            }

            let top: Vec<(u8, u8)> = (0..row_id).map(|r| (col_id, r)).rev().collect();
            let bottom: Vec<(u8, u8)> = (row_id + 1..=self.max_rows.try_into().unwrap())
                .map(|r| (col_id, r))
                .collect();
            let left: Vec<(u8, u8)> = (0..col_id).map(|c| (c, row_id)).rev().collect();
            let right: Vec<(u8, u8)> = (col_id + 1..=self.max_cols.try_into().unwrap())
                .map(|c| (c, row_id))
                .collect();

            let view_length = |surrondings: &Vec<(u8, u8)>| -> usize {
                let mut len = 0;
                for (col_id, row_id) in surrondings {
                    len += 1;
                    if self.get_pos(*col_id, *row_id) >= height {
                        break;
                    }
                }
                return len;
            };

            let view_lengths: [usize; 4] = [
                view_length(&top),
                view_length(&bottom),
                view_length(&left),
                view_length(&right),
            ];

            scenic_scores.push(view_lengths.iter().product());
        }

        return scenic_scores.iter().max().unwrap().clone();
    }
}
