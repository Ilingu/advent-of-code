pub mod cpu;
pub mod crt;

use std::fs;

use cpu::Cpu;
use crt::Crt;

fn main() {
    let raw_cpu_instructions = fs::read_to_string("./Assets/input.txt").unwrap();
    let cpu_instructions = raw_cpu_instructions.lines().collect::<Vec<&str>>();

    let mut cpu = Cpu::new();
    let mut crt = Crt::new();

    // closure argument can be improved to be more "cable-like":
    // `move |instruction: &str (fn to call on crt), args: Vec<unknown>|`
    // but since this is not a complete ctr, these params works just fine
    cpu.link_display(move |cycle, register, render: bool| match render {
        true => {
            crt.render_screen();
        }
        false => {
            crt.draw_pixel(cycle, register);
        }
    });

    // parse input
    cpu.exec(&cpu_instructions);

    // p1
    let p1 = cpu.compute_p1();
    println!("{p1}");

    // p2
    cpu.show_screen();
}
