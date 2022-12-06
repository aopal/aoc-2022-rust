use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    idx_first_n_unique(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    idx_first_n_unique(input, 14)
}

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
