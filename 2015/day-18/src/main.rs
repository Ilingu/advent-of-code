use std::fs;

#[derive(Clone, Copy, PartialEq, Debug)]
enum PixelState {
    Off,
    On,
}

impl From<char> for PixelState {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::On,
            '.' => Self::Off,
            _ => unreachable!("should not be possible, check your input"),
        }
    }
}

const STEPS: usize = 100;

type Screen = Vec<Vec<PixelState>>;

fn main() {
    // get input
    let input = fs::read_to_string("./src/input.txt").unwrap();

    // parse input
    let mut original_pixel_matrix: Screen = vec![vec![PixelState::Off; 100]; 100];
    for (y, line) in input.lines().enumerate() {
        for (x, pixel) in line.chars().enumerate() {
            original_pixel_matrix[y][x] = PixelState::from(pixel)
        }
    }

    // helper
    let get_no_of_on_pixels = |pm: &Screen| -> usize {
        pm.iter()
            .map(|state| {
                state
                    .iter()
                    .filter(|state| state == &&PixelState::On)
                    .count()
            })
            .sum()
    };

    // p1
    {
        let mut p1_pixel_matrix = original_pixel_matrix.clone();
        for _ in 0..STEPS {
            p1_pixel_matrix = next_animation_frame(&p1_pixel_matrix);
        }
        let p1 = get_no_of_on_pixels(&p1_pixel_matrix);
        println!("{p1}")
    }

    // p2
    {
        let set_stuck_pixel = |pm: &mut Screen| {
            pm[0][0] = PixelState::On;
            pm[original_pixel_matrix.len() - 1][original_pixel_matrix.len() - 1] = PixelState::On;
            pm[0][original_pixel_matrix.len() - 1] = PixelState::On;
            pm[original_pixel_matrix.len() - 1][0] = PixelState::On;
        };
        let mut p2_pixel_matrix = original_pixel_matrix.clone();
        set_stuck_pixel(&mut p2_pixel_matrix);

        for _ in 0..STEPS {
            p2_pixel_matrix = next_animation_frame(&p2_pixel_matrix);
            set_stuck_pixel(&mut p2_pixel_matrix)
        }
        let p2 = get_no_of_on_pixels(&p2_pixel_matrix);
        println!("{p2}")
    }
}

fn next_animation_frame(pixel_matrix: &Screen) -> Screen {
    let mut new_pixel_matrix: Screen = vec![vec![PixelState::Off; 100]; 100];
    for (y, line) in pixel_matrix.iter().enumerate() {
        for (x, pixel) in line.iter().enumerate() {
            let (x, y) = (x as isize, y as isize);
            let neighbord = [
                get_from_screen(pixel_matrix, (x - 1, y - 1)), // top left
                get_from_screen(pixel_matrix, (x, y - 1)),     // top middle
                get_from_screen(pixel_matrix, (x + 1, y - 1)), // top right
                get_from_screen(pixel_matrix, (x - 1, y)),     // middle left
                get_from_screen(pixel_matrix, (x + 1, y)),     // middle right
                get_from_screen(pixel_matrix, (x - 1, y + 1)), // bottom left
                get_from_screen(pixel_matrix, (x, y + 1)),     // bottom middle
                get_from_screen(pixel_matrix, (x + 1, y + 1)), // bottom right
            ];
            let on_neighbord = neighbord
                .iter()
                .filter(|state| state == &&PixelState::On)
                .count();

            let new_state = match pixel {
                PixelState::Off => {
                    if on_neighbord == 3 {
                        PixelState::On
                    } else {
                        PixelState::Off
                    }
                }
                PixelState::On => {
                    if on_neighbord == 2 || on_neighbord == 3 {
                        PixelState::On
                    } else {
                        PixelState::Off
                    }
                }
            };
            new_pixel_matrix[y as usize][x as usize] = new_state
        }
    }
    new_pixel_matrix
}

fn get_from_screen(pixel_matrix: &Screen, (x, y): (isize, isize)) -> PixelState {
    if x < 0 || y < 0 || x >= pixel_matrix.len() as isize || y >= pixel_matrix.len() as isize {
        return PixelState::Off;
    }
    pixel_matrix[y as usize][x as usize]
}
