#[derive(Debug)]
struct SectionAssignment {
    low: i32,
    high: i32,
}

impl SectionAssignment {
    fn new_from_str(line: String) -> Self {
        let nums: Vec<&str> = line.split("-").collect();

        Self {
            low: str::parse::<i32>(nums[0]).unwrap(),
            high: str::parse::<i32>(nums[1]).unwrap(),
        }
    }

    fn contains(&self, other: &SectionAssignment) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    fn intersects(&self, other: &SectionAssignment) -> bool {
        (self.low >= other.low && self.low <= other.high) ||
        (self.high >= other.low && self.low <= other.high)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let assignments: Vec<&str> = line.split(",").collect();

        let assignment1 = SectionAssignment::new_from_str(String::from(assignments[0]));
        let assignment2 = SectionAssignment::new_from_str(String::from(assignments[1]));

        if assignment1.contains(&assignment2) ||  assignment2.contains(&assignment1) {
            sum += 1;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let assignments: Vec<&str> = line.split(",").collect();

        let assignment1 = SectionAssignment::new_from_str(String::from(assignments[0]));
        let assignment2 = SectionAssignment::new_from_str(String::from(assignments[1]));

        if assignment1.intersects(&assignment2) {
            sum += 1;
        }
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(5));
    }
}
