pub mod cmd;
pub mod fs_tree;

use std::fs;

use cmd::parse_line;
use fs_tree::FileSystem;

fn main() {
    let raw_cmds = fs::read_to_string("./Assets/input.txt").unwrap();
    let mut cmds = Vec::from_iter(raw_cmds.lines());
    cmds.remove(0);

    let mut file_system = FileSystem::new();

    for line in cmds {
        println!("{line}");
        match parse_line(line) {
            (Some(file), None, None) => file_system.make_file(file),
            (None, Some(dir), None) => file_system.make_dir(dir),
            (None, None, Some(cmd)) => file_system.exec_cmd(cmd),
            _ => panic!("couldn't parse"),
        }
    }

    file_system.compute_fs_total_size();
    // println!("{:#?}", file_system);

    let p1 = file_system.solve_p1();
    let p2 = file_system.solve_p2();
    println!("{p1}");
    println!("{p2}")
}
