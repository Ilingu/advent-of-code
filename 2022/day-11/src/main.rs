pub mod monkey;

use std::fs;

use monkey::Monkey;

fn main() {
    let input = fs::read_to_string("./Assets/input.txt").unwrap();
    let mut monkeys = input
        .split("\n\n")
        .map(|mk| Monkey::from_string(mk))
        .collect::<Vec<Monkey>>();
    let mut monkey_throw: Vec<Monkey> = vec![Monkey::new(); monkeys.len()];
    let supermod = monkeys.iter().map(|x| x.test).product();

    for _ in 0..10000 {
        for (mk_id, mk) in monkeys.iter_mut().enumerate() {
            mk.items_worry
                .append(&mut monkey_throw.get_mut(mk_id).unwrap().items_worry);
            monkey_throw.get_mut(mk_id).unwrap().items_worry = vec![];

            while mk.items_worry.len() != 0 {
                let (throw_at, worry_lvl) = mk.inspect(0, supermod);
                monkey_throw
                    .get_mut(throw_at)
                    .unwrap()
                    .items_worry
                    .push(worry_lvl);
                mk.items_worry.remove(0);
            }
        }
    }

    let mut mk_inspected = monkeys
        .iter()
        .map(|x| x.inspected_no)
        .collect::<Vec<usize>>();
    mk_inspected.sort_by(|a, b| b.cmp(a));
    let most_active = &mk_inspected[0..2];

    // p1/p2
    println!("{}", most_active.iter().product::<usize>())
}
