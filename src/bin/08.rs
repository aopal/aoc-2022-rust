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
        for j in range.clone() {
            // left to right
            let curr_char = grid[i].chars().nth(j).unwrap();
            if curr_char > max_seen {
                max_seen = curr_char;
                trees_visible.insert((i, j));
            }
        }

        let mut max_seen = ' ';
        for j in range.clone().rev() {
            // right to left
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
        for i in range.clone() {
            // top to bottom
            let curr_char = grid[i].chars().nth(j).unwrap();
            if curr_char > max_seen {
                max_seen = curr_char;
                trees_visible.insert((i, j));
            }
        }

        let mut max_seen = ' ';
        for i in range.clone().rev() {
            // bottom to top
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
    let mut grid = Vec::<String>::new();
    for line in input.lines() {
        grid.push(line.to_string());
    }

    let mut max_visible = 0;
    let mut max_at = (0, 0);
    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            let visible = visible_from(&grid, (i, j));

            if visible >= max_visible {
                max_visible = visible;
                max_at = (i, j);
            }
        }
    }

    println!("max visible from {:?}", max_at);
    Some(max_visible)
}

fn visible_from(grid: &Vec<String>, point: (usize, usize)) -> u32 {
    let char_at_point = grid[point.0].chars().nth(point.1).unwrap();
    // let range_x = 0..point.0;
    // let range_y = 0..point.1;

    let mut n_visible: [u32; 4] = [0; 4];

    let j = point.1;
    for i in (0..point.0).rev() {
        n_visible[0] += 1;

        let c = grid[i].chars().nth(j).unwrap();

        if c >= char_at_point {
            break;
        }
    }

    for i in point.0 + 1..grid.len() {
        n_visible[1] += 1;

        let c = grid[i].chars().nth(j).unwrap();

        if c >= char_at_point {
            break;
        }
    }

    let i = point.0;
    for j in (0..point.1).rev() {
        n_visible[2] += 1;

        let c = grid[i].chars().nth(j).unwrap();

        if c >= char_at_point {
            break;
        }
    }

    for j in point.1 + 1..grid.len() {
        n_visible[3] += 1;

        let c = grid[i].chars().nth(j).unwrap();

        if c >= char_at_point {
            break;
        }
    }

    n_visible[0] * n_visible[1] * n_visible[2] * n_visible[3]
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
