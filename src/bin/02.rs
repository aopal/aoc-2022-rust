use std::{collections::HashMap};

pub fn part_one(input: &str) -> Option<u32> {
    calc_score(input, init_mappings1())
}

pub fn part_two(input: &str) -> Option<u32> {
    calc_score(input, init_mappings2())
}

fn calc_score(input: &str, score_map: HashMap<String, u32>) -> Option<u32> {
    let mut score_sum: u32 = 0;
    for line in input.lines() {
        let key = line.replace(" ", "");

        score_sum += score_map[&key];
    }

    Some(score_sum)
}

fn init_mappings1() -> HashMap<String, u32> {
    let mut score_map: HashMap<String, u32> = HashMap::new();

    score_map.insert("AX".to_string(), 3+1); // 3+1 = score for tying + score for playing rock
    score_map.insert("AY".to_string(), 6+2);
    score_map.insert("AZ".to_string(), 0+3);
    score_map.insert("BX".to_string(), 0+1);
    score_map.insert("BY".to_string(), 3+2);
    score_map.insert("BZ".to_string(), 6+3);
    score_map.insert("CX".to_string(), 6+1);
    score_map.insert("CY".to_string(), 0+2);
    score_map.insert("CZ".to_string(), 3+3);

    score_map
}

fn init_mappings2() -> HashMap<String, u32> {
    let mut score_map: HashMap<String, u32> = HashMap::new();

    score_map.insert("AX".to_string(), 0+3); // 0+3 = score for losing + score for playing scissors
    score_map.insert("AY".to_string(), 3+1);
    score_map.insert("AZ".to_string(), 6+2);
    score_map.insert("BX".to_string(), 0+1);
    score_map.insert("BY".to_string(), 3+2);
    score_map.insert("BZ".to_string(), 6+3);
    score_map.insert("CX".to_string(), 0+2);
    score_map.insert("CY".to_string(), 3+3);
    score_map.insert("CZ".to_string(), 6+1);

    score_map
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
