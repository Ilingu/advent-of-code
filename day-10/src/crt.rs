use std::collections::HashMap;

#[derive(Debug)]
pub struct Crt {
    screen: HashMap<(usize, usize), char>,
}

impl Crt {
    pub fn new() -> Self {
        Self {
            screen: HashMap::new(),
        }
    }

    pub fn draw_pixel(&mut self, cycle: usize, register: i32) {
        let sprite_pos = (register - 1)..=(register + 1);
        let screen_pos = ((cycle - 1) % 40, (cycle - 1) / 40);
        let is_lit = sprite_pos.contains(&(screen_pos.0 as i32));

        self.screen
            .insert(screen_pos, if is_lit { '#' } else { ' ' }); // should be '.', but a white space is more visual~~
    }

    pub fn render_screen(&self) -> String {
        let mut screen_pixels: Vec<Vec<char>> = vec![vec!['#'; 40]; 6];
        for ((x, y), p) in &self.screen {
            screen_pixels[*y][*x] = *p;
        }

        let mut screen = String::new();
        for ypixels in screen_pixels {
            for xpixel in ypixels {
                screen += &xpixel.to_string();
            }
            screen += "\n"
        }

        println!("{screen}");
        return screen;
    }
}
