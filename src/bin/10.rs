fn calc_x_vals(input: &str) -> Vec<i32> {
    let mut v = Vec::<i32>::new();
    v.push(1);

    let mut cycle = 1;
    for line in input.lines() {
        let curr_x = v[cycle-1];
        if line == "noop" {
            v.push(curr_x);
            cycle += 1;
        } else {
            let (_, n) = line.split_once(' ').unwrap();

            v.push(curr_x);
            v.push(curr_x + n.parse::<i32>().unwrap());
            cycle += 2;
        }
    }

    v
}

pub fn part_one(input: &str) -> Option<u32> {
    let vals = calc_x_vals(input);

    let mut sum: i32 = 0;
    for i in [20,60,100,140,180,220] {
        sum += vals[i-1] * i as i32;
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let vals = calc_x_vals(input);

    for (i, v) in vals.iter().enumerate() {
        let mut c = '⬛';
        if (i as i32 % 40 - v).abs() <= 1 {
            c = '⬜';
        }

        print!("{}", c);
        if i % 40 == 39 {
            print!("\n");
        }
    }
    Some(1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
