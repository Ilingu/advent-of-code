use std::fs;

fn main() {
    let input = fs::read_to_string("./Assets/baseinput.txt").unwrap();

    let total_amount = input
        .lines()
        .map(|snafu_num| snafu_to_base10(snafu_num))
        .sum();
    println!("{}", base10_to_snafu(total_amount))
}

fn snafu_to_base10(snafu: &str) -> isize {
    let mut output = 0;
    for (slot, snafu_digit) in snafu.chars().rev().enumerate() {
        let digit = match snafu_digit {
            '-' => -1,
            '=' => -2,
            _ => snafu_digit.to_digit(10).unwrap() as isize,
        };
        output += digit * 5_isize.pow(slot as u32);
    }
    output
}

fn base10_to_snafu(mut base10: isize) -> String {
    let mut out = String::new();
    while base10 > 0 {
        let rem = ((base10 + 2) % 5) - 2;
        base10 = (base10 + 2) / 5;

        out.push(match rem {
            -2 => '=',
            -1 => '-',
            _ => char::from_digit(rem as u32, 10).unwrap(),
        });
    }
    out.chars().rev().collect()
}
