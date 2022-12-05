use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
struct Stack {
    data: VecDeque<char>,
}

impl Stack {
    fn new() -> Self {
        Self {
            data: VecDeque::<char>::new(),
        }
    }

    fn push_one(&mut self, c: char) {
        self.data.push_back(c);
    }

    fn push(&mut self, new_data: Vec<char>) {
        for datum in new_data.into_iter() {
            self.data.push_back(datum);
        }
    }

    fn pop(&mut self, n: u32) -> Vec<char> {
        let mut out = Vec::<char>::new();
        out.reserve(n as usize);

        for _ in 0..n {
            let d = self.data.pop_back();
            if d.is_some() {
                out.push(d.unwrap());
            }
        }

        out
    }

    fn top(&self) -> char {
        self.data.get(self.data.len() - 1).unwrap().to_owned()
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[");

        for c in self.data.iter() {
            write!(f, "{}", c);
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
struct ShippingYard {
    stacks: Vec<Stack>,
}

impl ShippingYard {
    fn new() -> Self {
        Self {
            stacks: Vec::<Stack>::new(),
        }
    }

    fn init(&mut self, initial_stacks: Vec<String>) {
        self.stacks.reserve(initial_stacks.len());

        for v in initial_stacks.iter() {
            let mut stack = Stack::new();

            for c in v.chars() {
                stack.push_one(c);
            }

            self.stacks.push(stack)
        }
    }

    fn move_n(&mut self, mut n: u32, from: usize, to: usize) {
        if n as usize > self.stacks[from].data.len() {
            n = self.stacks[from].data.len() as u32
        }

        let new_data = self.stacks[from].pop(n);
        self.stacks[to].push(new_data);
    }

    fn move_n_reverse(&mut self, mut n: u32, from: usize, to: usize) {
        if n as usize > self.stacks[from].data.len() {
            n = self.stacks[from].data.len() as u32
        }

        let mut new_data = self.stacks[from].pop(n);
        new_data.reverse();
        self.stacks[to].push(new_data);
    }

    fn read_tops(&self) -> Vec<char> {
        let mut tops = Vec::<char>::new();
        tops.reserve(self.stacks.len());

        for stack in self.stacks.iter() {
            tops.push(stack.top())
        }

        tops
    }
}

impl fmt::Display for ShippingYard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, stack) in self.stacks.iter().enumerate() {
            write!(f, "{}: {}\n", i + 1, stack);
        }

        write!(f, "")
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut yard_input = Vec::<String>::new();
    let mut yard = ShippingYard::new();

    loop {
        let line = lines.next().unwrap();
        if line.len() == 0 {
            break;
        }

        yard_input.push(String::from(line))
    }

    yard.init(yard_input);
    println!("start:\n{}", yard);

    loop {
        let l = lines.next();
        if l.is_none() {
            break;
        }

        let cmd: Vec<&str> = l.unwrap().split(" ").collect();
        let n = str::parse::<u32>(cmd[0]).unwrap();
        let from = str::parse::<usize>(cmd[1]).unwrap();
        let to = str::parse::<usize>(cmd[2]).unwrap();

        yard.move_n(n, from - 1, to - 1);
    }
    println!("end:\n{}", yard);

    for c in yard.read_tops() {
        print!("{}", c);
    }
    print!("\n");

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut yard_input = Vec::<String>::new();
    let mut yard = ShippingYard::new();

    loop {
        let line = lines.next().unwrap();
        if line.len() == 0 {
            break;
        }

        yard_input.push(String::from(line))
    }

    yard.init(yard_input);
    println!("start:\n{}", yard);

    loop {
        let l = lines.next();
        if l.is_none() {
            break;
        }

        let cmd: Vec<&str> = l.unwrap().split(" ").collect();
        let n = str::parse::<u32>(cmd[0]).unwrap();
        let from = str::parse::<usize>(cmd[1]).unwrap();
        let to = str::parse::<usize>(cmd[2]).unwrap();

        yard.move_n_reverse(n, from - 1, to - 1);
    }
    println!("end:\n{}", yard);

    for c in yard.read_tops() {
        print!("{}", c);
    }
    print!("\n");

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 35);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 35);
        assert_eq!(part_two(&input), None);
    }
}
