use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Vec::<String>::new();
    for line in input.lines() {
        grid.push(line.to_string());
    }

    let mut trees_visible = HashSet::<(usize, usize)>::new();
    let range = 0..grid.len();

    // count for rows
    for i in range.clone() {
        let mut max_seen = ' ';
        for j in range.clone() {// left to right
            let curr_char = grid[i].chars().nth(j).unwrap();
            if curr_char > max_seen {
                max_seen = curr_char;
                trees_visible.insert((i, j));
            }
        }

        let mut max_seen = ' ';
        for j in range.clone().rev() {// right to left
            let curr_char = grid[i].chars().nth(j).unwrap();
            if curr_char > max_seen {
                max_seen = curr_char;
                trees_visible.insert((i, j));
            }
        }
    }

    // count for cols
    for j in range.clone() {
        let mut max_seen = ' ';
        for i in range.clone() {// top to bottom
            let curr_char = grid[i].chars().nth(j).unwrap();
            if curr_char > max_seen {
                max_seen = curr_char;
                trees_visible.insert((i, j));
            }
        }

        let mut max_seen = ' ';
        for i in range.clone().rev() {// bottom to top
            let curr_char = grid[i].chars().nth(j).unwrap();
            if curr_char > max_seen {
                max_seen = curr_char;
                trees_visible.insert((i, j));
            }
        }
    }

    // println!("{} {:?}", trees_visible.len(), trees_visible);
    Some(trees_visible.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
