use crate::fs_tree::{FSDirNode, FSFileNode};

#[derive(Debug)]
pub enum Cmds {
    Ls,
    Cd,
}

#[derive(Debug)]
pub struct Command {
    pub cmd: Cmds,
    pub arg: Option<String>,
}

impl Command {
    pub fn new(line: String) -> Self {
        let parsed_cmd = Vec::from_iter(line.trim_start_matches("$ ").split_whitespace());
        if parsed_cmd.len() < 1 || parsed_cmd.len() > 2 {
            panic!("parsed_cmd.len() <= 0")
        }

        let tcmd: Cmds = match parsed_cmd[0] {
            "cd" => Cmds::Cd,
            "ls" => Cmds::Ls,
            _ => panic!("invalid cmd"),
        };

        let mut arg: Option<String> = None;
        if parsed_cmd.len() == 2 {
            arg = Some(parsed_cmd[1].to_string());
        }

        return Self { cmd: tcmd, arg };
    }
}

pub fn parse_line(cmd: &str) -> (Option<FSFileNode>, Option<FSDirNode>, Option<Command>) {
    if cmd.starts_with("dir") {
        return (
            None,
            Some(FSDirNode::new(cmd.trim_start_matches("dir ").to_string())),
            None,
        );
    }

    if cmd.starts_with("$") {
        return (None, None, Some(Command::new(cmd.to_string())));
    }

    let file = match FSFileNode::from_line(cmd) {
        Some(file_node) => file_node,
        None => return (None, None, None),
    };

    return (Some(file), None, None);
}
