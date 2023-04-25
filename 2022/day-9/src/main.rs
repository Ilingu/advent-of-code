pub mod rope;

use std::fs;

use rope::Rope;

/*
My first approach of this problem was to move the tail to each last position of the head whenever they was a delta > 1 between the head and the tail, that worked very well with a under 30 lines of code, I was pround... until part 2 that introduced new movements... so day-9 was for me a complete rewrite because of my lasiness... 😭
*/

fn main() {
    let moves = fs::read_to_string("./Assets/input.txt").unwrap();

    let mut rope_p1 = Rope::new(2);
    let mut rope_p2 = Rope::new(10);

    for mvt in moves.lines() {
        let args = mvt.split_whitespace().collect::<Vec<&str>>();
        if args.len() != 2 {
            panic!("args.len() != 2")
        }

        let mvt_instruction = (args[0], args[1].parse::<usize>().unwrap());
        rope_p1.move_head(mvt_instruction);
        rope_p2.move_head(mvt_instruction);
    }

    // println!("{:#?}", grid);

    let p1 = rope_p1.tail_history_len();
    let p2 = rope_p2.tail_history_len();
    println!("{p1}");
    println!("{p2}");
}

// for p2, remember that each knots follow the pos of its prev knots
