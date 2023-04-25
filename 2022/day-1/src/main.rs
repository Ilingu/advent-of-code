use std::fs;

fn main() {
    let all_calories = fs::read_to_string("./src/input.txt").unwrap();
    let elfs_calories = all_calories.split("\n\n");

    let mut elf_max_sum = 0;
    let mut elfs_sums: Vec<i32> = vec![];

    for elf_calories in elfs_calories {
        let elf_calories_arr = elf_calories.split("\n");

        let mut sums = 0;
        for elf_calories in elf_calories_arr {
            let item_calories = elf_calories.parse::<i32>().unwrap();
            sums += item_calories
        }

        if sums > elf_max_sum {
            elf_max_sum = sums
        }
        elfs_sums.push(sums)
    }

    // Challenge 1
    println!("{elf_max_sum}");

    // Challenge 2
    elfs_sums.sort_by(|a, b| b.cmp(a));
    let elfs_top3 = &elfs_sums[0..3];
    println!("{:?}", elfs_top3);

    let sums_top3: i32 = elfs_top3.iter().sum();
    println!("{sums_top3}")
}
