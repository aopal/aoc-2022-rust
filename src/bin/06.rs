use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct UniqueSet {
    num_chars_duplicated: u32,
    ref_counts: HashMap<char, u32>,
    queue: VecDeque<char>,
    size: usize,
}

impl UniqueSet {
    fn new(size: usize) -> Self {
        Self {
            num_chars_duplicated: 0,
            ref_counts: HashMap::<char, u32>::new(),
            queue: VecDeque::<char>::new(),
            size: size,
        }
    }

    fn all_unique(&self) -> bool {
        if self.queue.len() < self.size {
            false
        } else {
            self.num_chars_duplicated == 0
        }
    }

    fn insert(&mut self, c: char) -> bool {
        self.queue.push_back(c);
        self.ref_counts.entry(c).and_modify(|count| *count += 1).or_insert(1);

        // if this insertion moved the reference count up from 1
        if self.ref_counts[&c] == 2 {
            self.num_chars_duplicated += 1
        }

        if self.queue.len() > self.size {
            let c_removed = self.queue.pop_front().unwrap();
            self.ref_counts.entry(c_removed).and_modify(|count| *count -= 1);

            // if this deletion moved the reference count down to 1
            if self.ref_counts[&c_removed] == 1 {
                self.num_chars_duplicated -= 1;
            }
        }

        self.all_unique()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    idx_first_n_unique_fast(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    idx_first_n_unique_fast(input, 14)
}

fn idx_first_n_unique_fast(input: &str, n: usize) -> Option<u32> {
    let mut set = UniqueSet::new(n);

    for (i, c) in input.chars().enumerate() {
        if set.insert(c) {
            return Some((i+1) as u32)
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}


// old solution
fn idx_first_n_unique(input: &str, n: usize) -> Option<u32> {
    let mut last_n_chars = Vec::<char>::with_capacity(n);
    for _ in 0..n {
        last_n_chars.push(' ');
    }

    for (i, c) in input.chars().enumerate() {
        last_n_chars[i%n] = c;

        if all_unique(&last_n_chars, &n) {
            return Some((i + 1) as u32)
        }
    }

    None
}

fn all_unique(chars: &Vec<char>, n: &usize) -> bool {
    if chars[n-1] == ' ' {
        return false
    }

    let mut charset = HashSet::<char>::new();

    for c in chars.into_iter() {
        if charset.contains(c) {
            return false
        }

        charset.insert(*c);
    }

    true
}
