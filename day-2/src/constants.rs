pub const ENEMY_POSSIBLE_CHOICES: [&str; 3] = ["A", "B", "C"];
pub const MY_POSSIBLE_CHOICES: [&str; 3] = ["X", "Y", "Z"];

pub enum Outcome {
    WIN,
    DRAW,
    LOSE,
}

pub enum Action {
    Rock,
    Paper,
    Scissor,
}

pub enum Option<T> {
    Ok(T),
    None,
}

impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(t) => t,
            Self::None => panic!("invalid result"),
        }
    }
}
