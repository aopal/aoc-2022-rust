pub fn part_one(input: &str) -> Option<u32> {
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut max_cals: u32 = 0;
    let mut max_elf: u32 = 0;
    let mut i: u32 = 0;

    for elf in elves.iter() {
        let cals = elf.lines();
        let mut curr_sum: u32 = 0;
        for cal in cals {
            curr_sum += cal.parse::<u32>().unwrap()
        }

        if curr_sum > max_cals {
            max_cals = curr_sum;
            max_elf = i
        }
        i += 1;
    }

    return Option::<u32>::from(max_cals);
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves: Vec<&str> = input.split("\n\n").collect();

    let mut sums: Vec<u32> = elves.into_iter().map(|elf| {
        let cals = elf.lines();
        let mut curr_sum: u32 = 0;
        for cal in cals {
            curr_sum += cal.parse::<u32>().unwrap()
        }
        curr_sum
    }).collect();

    sums.sort_by(|a, b| b.cmp(a));

    let final_sum = sums[0] + sums[1] + sums[2];

    return Option::<u32>::from(final_sum);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
