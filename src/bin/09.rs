use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

struct Instruction {
    steps_remaining: u32,
    velocity: (i32, i32),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, n) = s.split_once(' ').unwrap();

        let steps = n.parse::<u32>().unwrap();
        let vec: (i32, i32);
        vec = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => (0, 0),
        };

        Ok(Instruction {
            steps_remaining: steps,
            velocity: vec,
        })
    }
}

impl Instruction {
    fn consume(&mut self) -> Option<(i32, i32)> {
        if self.steps_remaining == 0 {
            None
        } else {
            self.steps_remaining -= 1;
            Some(self.velocity)
        }
    }
}

fn simulate_rope(input: &str, n: usize) -> Option<u32> {
    let mut points_visited = HashSet::<(i32, i32)>::new();

    let mut positions = Vec::<(i32, i32)>::new();
    positions.resize_with(n, Default::default);

    let tail_idx = n - 1;
    points_visited.insert(positions[tail_idx]);

    for line in input.lines() {
        let mut instr = Instruction::from_str(line).unwrap();

        loop {
            let vec_opt = instr.consume();
            if vec_opt.is_none() {
                break;
            }

            let head_vel = vec_opt.unwrap();

            positions[0].0 += head_vel.0;
            positions[0].1 += head_vel.1;

            for i in 1..n {
                let mut vec_del = (
                    positions[i - 1].0 - positions[i].0,
                    positions[i - 1].1 - positions[i].1,
                );

                // if points are adjacent or overlapping
                if vec_del.0.abs() <= 1 && vec_del.1.abs() <= 1 {
                    continue;
                }

                if vec_del.0 == 2 {
                    vec_del.0 = 1;
                }
                if vec_del.0 == -2 {
                    vec_del.0 = -1;
                }

                if vec_del.1 == 2 {
                    vec_del.1 = 1;
                }
                if vec_del.1 == -2 {
                    vec_del.1 = -1;
                }

                positions[i].0 += vec_del.0;
                positions[i].1 += vec_del.1;
            }

            points_visited.insert(positions[tail_idx]);
        }
    }

    Some(points_visited.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    simulate_rope(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    simulate_rope(input, 10)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
