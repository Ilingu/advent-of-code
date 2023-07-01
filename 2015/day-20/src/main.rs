const INPUT: usize = 36000000;
use std::{thread, time::Instant};

fn main() {
    let start_time = Instant::now();

    let mut workers = vec![];
    workers.push(thread::spawn(|| {
        let mut p1 = 1;
        while present_p1(p1) < INPUT {
            p1 += 1;
        }
        format!("part 1: {p1}")
    }));
    workers.push(thread::spawn(|| {
        let mut p2 = 1;
        while present_p2(p2) < INPUT {
            p2 += 1;
        }
        format!("part 2: {p2}")
    }));

    for worker in workers {
        println!("{}", worker.join().unwrap());
    }
    println!("time = {:?}", start_time.elapsed());
}

fn get_divisors(n: usize) -> Vec<usize> {
    vec![vec![1, n], divisors::get_divisors(n)].concat()
}

fn present_p1(house_number: usize) -> usize {
    let elfs_id = get_divisors(house_number);
    elfs_id.iter().map(|&d| d * 10).sum()
}

fn present_p2(house_number: usize) -> usize {
    let elfs_id = get_divisors(house_number);
    elfs_id
        .iter()
        .filter(|&&d| d * 50_usize > house_number)
        .map(|&d| d * 11)
        .sum()
}
