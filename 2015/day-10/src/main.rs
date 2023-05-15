const INPUT: &str = "3113322113";

fn main() {
    let forty = look_and_say(INPUT, 40);

    let p1 = forty.len();
    println!("{p1}");

    let p2 = look_and_say(&forty, 10).len();
    println!("{p2}");
}

fn look_and_say(base: &str, n: usize) -> String {
    let mut input = base.to_owned();
    for _ in 0..n {
        let mut new_input = String::new();
        let (mut num_of_digit, mut last_num) = (0, input.chars().nth(0).unwrap());
        for ch in input.chars() {
            match ch == last_num {
                true => num_of_digit += 1,
                false => {
                    new_input.push_str(format!("{num_of_digit}{last_num}").as_str());
                    last_num = ch;
                    num_of_digit = 1
                }
            }
        }
        new_input.push_str(format!("{num_of_digit}{last_num}").as_str());
        input = new_input
    }
    input
}
