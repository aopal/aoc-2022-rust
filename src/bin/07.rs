use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq)]
enum NodeType {
    Dir,
    File,
}

#[derive(Debug)]
struct Node {
    kind: NodeType,
    name: String,
    index: usize,
    children: HashMap<String, usize>,
    total_size: u32,
}

impl Node {
    fn new_dir(name: String,index: usize) -> Self {
        Self {
            kind: NodeType::Dir,
            name: name,
            index: index,
            children: HashMap::new(),
            total_size: 0,
        }
    }

    fn new_file(name: String, index: usize, size: u32) -> Self {
        Self {
            kind: NodeType::File,
            name: name,
            index: index,
            children: HashMap::new(),
            total_size: size,
        }
    }

    fn pretty_print(&self, ptrs: &Vec<Node>, indent: u32) {
        print!("{}- {}", " ".repeat(indent as usize * 2), self.name);
        if self.kind == NodeType::Dir {
            print!(" (dir, size={})\n", self.total_size)
        } else if self.kind == NodeType::File {
            print!(" (file, size={})\n", self.total_size)
        }

        for (_, child_idx) in self.children.iter() {
            ptrs[*child_idx].pretty_print(ptrs, indent + 1);
        }
    }
}

pub fn day_seven(input: &str) -> Option<u32> {
    // define regexes
    let root_re = Regex::new(r"^\$ cd /$").unwrap();
    let ls_re = Regex::new(r"^\$ ls$").unwrap();

    let cd_re = Regex::new(r"^\$ cd (.+)$").unwrap();
    let cd_parent_re = Regex::new(r"^\$ cd \.\.$").unwrap();
    let dir_re = Regex::new(r"^dir (.+)$").unwrap();
    let file_re = Regex::new(r"^(\d+) (.+)$").unwrap();

    // common, build directory tree
    let root_node = Node::new_dir("/".to_string(), 0);
    let mut dir_stack = VecDeque::<usize>::new();
    let mut node_pointers = Vec::<Node>::new();
    node_pointers.push(root_node);
    dir_stack.push_front(0);

    let mut next_index: usize = 1;

    for line in input.lines() {
        let cwd = &mut (node_pointers[dir_stack[0]]);

        if root_re.is_match(line) || ls_re.is_match(line) {
            continue;
        } else if cd_parent_re.is_match(line) {
            dir_stack.pop_front();
        } else if cd_re.is_match(line) {
            let dir_name = cd_re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
            dir_stack.push_front(cwd.children[&dir_name])
        } else if dir_re.is_match(line) {
            let dir_name = dir_re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
            let new_dir = Node::new_dir(dir_name.clone(), next_index);
            cwd.children.insert(dir_name.clone(), new_dir.index);
            node_pointers.push(new_dir);

            next_index += 1;
        } else if file_re.is_match(line) {
            let captures = file_re.captures(line).unwrap();

            let file_size = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let file_name = captures.get(2).unwrap().as_str().to_string();

            let new_file = Node::new_file(file_name.clone(), next_index, file_size);
            cwd.children.insert(file_name.clone(), new_file.index);
            node_pointers.push(new_file);
            next_index += 1;

            for node_idx in dir_stack.iter() {
                node_pointers[*node_idx].total_size += file_size;
            }
        }
    }

    node_pointers[0].pretty_print(&node_pointers, 0);

    // answer part 1
    let mut total_sum: u32 = 0;
    for node in node_pointers.iter() {
        if node.kind == NodeType::Dir && node.total_size <= 100_000 {
            total_sum += node.total_size;
        }
    }

    println!("\nPart 1 answer: {}\n", total_sum);

    // answer part 2
    let total_fs_size = 70000000;
    let target_fs_size = 30000000;
    let amount_to_free = target_fs_size - (total_fs_size - node_pointers[0].total_size);
    println!(
        "root size: {}, amount to free: {}",
        node_pointers[0].total_size, amount_to_free
    );

    let mut smallest_sufficient_dir_size = node_pointers[0].total_size;
    for node in node_pointers.iter() {
        if node.total_size >= amount_to_free && node.total_size < smallest_sufficient_dir_size {
            smallest_sufficient_dir_size = node.total_size;
        }
    }

    println!("\nPart 2 answer: {}\n", smallest_sufficient_dir_size);

    Some(total_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    day_seven(&input);
}
