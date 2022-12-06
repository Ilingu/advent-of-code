use std::fs;

fn main() {
    let raw_pairs = fs::read_to_string("./src/Assets/input.txt").unwrap();

    let pairs = Vec::from_iter(raw_pairs.lines());

    let mut p1 = 0;
    let mut p2 = 0;

    for pair in pairs {
        let pair_arr = Vec::from_iter(pair.split(","));
        if pair_arr.len() != 2 {
            panic!("len != 2")
        }

        let (elf1_min, elf1_max) = parse_interval(pair_arr[0]);
        let (elf2_min, elf2_max) = parse_interval(pair_arr[1]);

        if (elf1_min <= elf2_min && elf1_max >= elf2_max)
            || (elf2_min <= elf1_min && elf2_max >= elf1_max)
        {
            p1 += 1
        }

        let elf1 = elf1_min..(elf1_max + 1);
        let elf2 = elf2_min..(elf2_max + 1);
        for num in elf1.clone() {
            if elf2.contains(&num) {
                p2 += 1;
                break;
            }
        }
    }

    println!("{p1}");
    println!("{p2}");
}

fn parse_interval(raw_interval: &str) -> (u8, u8) {
    let minmax = Vec::from_iter(raw_interval.split("-"));
    if minmax.len() != 2 {
        panic!("len != 2")
    }

    return (
        minmax[0].parse::<u8>().unwrap(),
        minmax[1].parse::<u8>().unwrap(),
    );
}
