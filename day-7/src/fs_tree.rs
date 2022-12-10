use std::{collections::HashMap, vec};

use crate::cmd::{Cmds, Command};

#[derive(Debug)]
pub struct FSFileNode {
    pub size: i64,
    pub name: String,
}

impl FSFileNode {
    pub fn new(name: String, size: i64) -> Self {
        Self { name, size }
    }

    pub fn from_line(line: &str) -> Option<Self> {
        let parsed_file = Vec::from_iter(line.split_whitespace());
        if parsed_file.len() != 2 {
            return None;
        }

        let size = parsed_file[0].parse::<i64>().unwrap();
        Some(Self::new(parsed_file[1].to_string(), size))
    }
}

#[derive(Debug)]
pub struct FSDirNode {
    name: String,
    file_size: i64,
    total_size: i64,

    dirs: HashMap<String, FSDirNode>,
    files: Vec<FSFileNode>,
}

impl FSDirNode {
    pub fn new(name: String) -> Self {
        Self {
            name,
            file_size: 0,
            total_size: 0,
            dirs: HashMap::new(),
            files: vec![],
        }
    }

    pub fn compute_total_size(&mut self) -> Vec<i64> {
        let child_dirs = self.dirs.values_mut();
        if child_dirs.len() == 0 {
            self.total_size = self.file_size;
            return vec![self.file_size];
        }

        let mut all_file_size: Vec<i64> = vec![];
        for dir in child_dirs {
            all_file_size.append(&mut FSDirNode::compute_total_size(dir));
        }

        self.total_size = self.file_size + all_file_size.iter().sum::<i64>();
        return vec![self.total_size];
    }
}

#[derive(Debug)]
pub struct FileSystem {
    curr_path: String,
    root: FSDirNode,
}

impl FileSystem {
    // Public

    pub fn new() -> Self {
        Self {
            curr_path: "/".to_string(),
            root: FSDirNode::new("/".to_string()),
        }
    }

    pub fn exec_cmd(&mut self, cmd: Command) {
        match cmd.cmd {
            Cmds::Cd => self.cd(&cmd.arg.unwrap()),
            _ => {}
        }
    }

    pub fn pwd(&self) -> &str {
        &&self.curr_path
    }

    pub fn make_dir(&mut self, new_dir: FSDirNode) {
        println!("new dir: {}", new_dir.name);
        match self.find_as_mut(&self.pwd().to_owned()) {
            Some(dir) => dir.dirs.insert(new_dir.name.clone(), new_dir),
            None => panic!("couldn't create dir"),
        };
    }

    pub fn make_file(&mut self, new_file: FSFileNode) {
        println!("new file: {}", new_file.name);
        match self.find_as_mut(&self.pwd().to_owned()) {
            Some(dir) => {
                dir.file_size += new_file.size;
                dir.files.push(new_file);
            }
            None => panic!("couldn't create file"),
        };
    }

    pub fn compute_fs_total_size(&mut self) {
        self.root.compute_total_size();
    }

    pub fn solve_p1(&mut self) -> i64 {
        let mut all_sizes: Vec<i64> = vec![];
        let mut crawl_size = |dir: &FSDirNode| {
            all_sizes.push(dir.total_size);
        };

        Self::traverse(&self.root, &mut crawl_size);
        return all_sizes.iter().filter(|x| x <= &&100000).sum::<i64>();
    }

    pub fn solve_p2(&mut self) -> i64 {
        let mut all_sizes: Vec<i64> = vec![];
        let mut crawl_size = |dir: &FSDirNode| {
            all_sizes.push(dir.total_size);
        };

        Self::traverse(&self.root, &mut crawl_size);

        let unused_space = 70000000 - self.root.total_size;
        let space_to_free = 30000000 - unused_space;
        if space_to_free <= 0 {
            panic!("error doesn't come from the disk space 😉");
        }

        return *all_sizes
            .iter()
            .filter(|s| s >= &&space_to_free)
            .min()
            .unwrap();
    }

    // Private

    fn traverse<F>(curr_dir: &FSDirNode, cb: &mut F)
    where
        F: FnMut(&FSDirNode),
    {
        cb(curr_dir);
        for dir in curr_dir.dirs.values() {
            Self::traverse(dir, cb)
        }
    }

    fn cd(&mut self, path: &str) {
        match path {
            "/" => self.curr_path = "/".to_string(),
            ".." => {
                let paths = Vec::from_iter(self.curr_path.split("/"));
                let parent = paths[..(paths.len() - 1)].join("/");
                self.curr_path = parent
            }
            _ => {
                if path == "" {
                    panic!("invalid cd")
                }

                let mut new_path = self.pwd().to_string();
                if new_path.ends_with("/") {
                    new_path += &format!("{path}")
                } else {
                    new_path += &format!("/{path}")
                }

                match self.find_as_mut(&new_path) {
                    Some(_) => self.curr_path = new_path,
                    None => panic!("couldn't cd"),
                }
            }
        }
    }

    fn find_as_mut(&mut self, dir_path: &String) -> Option<&mut FSDirNode> {
        return Self::recursive_search(&mut self.root, dir_path.clone());
    }

    fn recursive_search(dir: &mut FSDirNode, path: String) -> Option<&mut FSDirNode> {
        let parsed_path = path.split("/").filter(|s| s != &"").collect::<Vec<&str>>();
        if parsed_path.len() <= 0 {
            return Some(dir);
        }

        let next_dir = dir.dirs.get_mut(&parsed_path.first().unwrap().to_string());

        if parsed_path.len() == 1 {
            return next_dir;
        }

        return match next_dir {
            Some(dir_node) => Self::recursive_search(dir_node, parsed_path[1..].join("/")),
            None => None,
        };
    }
}
