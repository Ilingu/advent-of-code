use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    let (mut wrapping_paper, mut ribons) = (0, 0);
    for box_dimension in input.lines() {
        let bd = box_dimension
            .split("x")
            .map(|d| d.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let (l, w, h) = (bd[0], bd[1], bd[2]);
        let areas = [l * w, w * h, h * l];

        let little_extra = *areas.iter().min().unwrap();
        let surface_area = areas.iter().map(|a| a * 2).sum::<u32>();

        let perimeters = vec![2 * l + 2 * h, 2 * l + 2 * w, 2 * w + 2 * h];
        wrapping_paper += surface_area + little_extra;
        ribons += perimeters.iter().min().unwrap() + l * w * h;
    }

    println!("{wrapping_paper}");
    println!("{ribons}");
}
