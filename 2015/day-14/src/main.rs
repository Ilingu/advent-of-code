use std::fs;

#[derive(Debug)]
enum State {
    Moving,
    Resting,
}

#[derive(Debug)]
struct Reindeer {
    speed: usize, // in km/s
    flying_time: usize,
    resting_time: usize,

    distance: usize,
    state: State,
    remaining_state_time: usize,
    score: usize,
}

impl Reindeer {
    fn parse_input(input: &str) -> Self {
        let datas = input.split_whitespace().collect::<Vec<_>>();
        Self {
            speed: datas[3].parse().unwrap(),
            flying_time: datas[6].parse().unwrap(),
            resting_time: datas[13].parse().unwrap(),

            distance: 0,
            state: State::Moving,
            remaining_state_time: datas[6].parse().unwrap(),
            score: 0,
        }
    }

    // old p1, way faster than update_pos(), but not efficient at all for part 2
    /*fn compute_max_distance(&self, nth_second: usize) -> usize {
        let full_cycle_count = nth_second / (self.flying_time + self.resting_time);
        let (sec_remaining, mut distance) = (
            nth_second.rem_euclid(self.flying_time + self.resting_time),
            full_cycle_count * self.flying_time * self.speed,
        );
        if sec_remaining >= self.flying_time {
            distance += self.flying_time * self.speed
        }
        distance
    }*/

    fn update_pos(&mut self) {
        match self.state {
            State::Moving => {
                self.distance += self.speed;
                if self.remaining_state_time == 1 {
                    self.remaining_state_time = self.resting_time;
                    self.state = State::Resting;
                } else {
                    self.remaining_state_time -= 1
                }
            }
            State::Resting => {
                if self.remaining_state_time == 1 {
                    self.remaining_state_time = self.flying_time;
                    self.state = State::Moving;
                } else {
                    self.remaining_state_time -= 1
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let mut reindeers = input.lines().map(Reindeer::parse_input).collect::<Vec<_>>();

    for _ in 0..2503 {
        reindeers.iter_mut().for_each(|r| r.update_pos());
        let max = reindeers.iter().map(|r| r.distance).max().unwrap();
        reindeers
            .iter_mut()
            .filter(|r| r.distance == max)
            .for_each(|r| r.score += 1)
    }

    let p1 = reindeers.iter().map(|r| r.distance).max().unwrap();
    println!("{p1}");

    let p2 = reindeers.iter().map(|r| r.score).max().unwrap();
    println!("{p2}");
}
