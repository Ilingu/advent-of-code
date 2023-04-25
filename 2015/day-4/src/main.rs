const SECRET_KEY: &str = "iwrupvqb";

fn main() {
    let p1 = first_n_zeroes(5);
    println!("{p1}");
    let p2 = first_n_zeroes(6);
    println!("{p2}");
}

fn first_n_zeroes(n: usize) -> usize {
    let mut num: usize = 0;
    while !md5_hash(format!("{SECRET_KEY}{num}").as_str()).starts_with(&"0".repeat(n)) {
        num += 1
    }
    num
}

fn md5_hash(input: &str) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}
