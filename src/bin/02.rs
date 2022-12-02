use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    calc_score(input, init_mappings1())
}

pub fn part_two(input: &str) -> Option<u32> {
    calc_score(input, init_mappings2())
}

fn calc_score(input: &str, score_map: HashMap<String, u32>) -> Option<u32> {
    let mut score_sum: u32 = 0;
    for line in input.lines() {
        score_sum += score_map[line];
    }

    Some(score_sum)
}

fn init_mappings1() -> HashMap<String, u32> {
    let mut score_map: HashMap<String, u32> = HashMap::new();

    score_map.insert("A X".to_string(), 3+1); // 3+1 = score for tying + score for playing rock
    score_map.insert("A Y".to_string(), 6+2);
    score_map.insert("A Z".to_string(), 0+3);
    score_map.insert("B X".to_string(), 0+1);
    score_map.insert("B Y".to_string(), 3+2);
    score_map.insert("B Z".to_string(), 6+3);
    score_map.insert("C X".to_string(), 6+1);
    score_map.insert("C Y".to_string(), 0+2);
    score_map.insert("C Z".to_string(), 3+3);

    score_map
}

fn init_mappings2() -> HashMap<String, u32> {
    let mut score_map: HashMap<String, u32> = HashMap::new();

    score_map.insert("A X".to_string(), 0+3); // 0+3 = score for losing + score for playing scissors
    score_map.insert("A Y".to_string(), 3+1);
    score_map.insert("A Z".to_string(), 6+2);
    score_map.insert("B X".to_string(), 0+1);
    score_map.insert("B Y".to_string(), 3+2);
    score_map.insert("B Z".to_string(), 6+3);
    score_map.insert("C X".to_string(), 0+2);
    score_map.insert("C Y".to_string(), 3+3);
    score_map.insert("C Z".to_string(), 6+1);

    score_map
}

pub fn part_one_mod(input: &str) -> Option<u32> {
    let mut score_sum: u32 = 0;

    for line in input.lines() {
        // 65 is the ASCII offset of A, 88 is the offset of X
        let opponent_char = line.chars().nth(0).unwrap() as i32 - 65;
        let player_char = line.chars().nth(2).unwrap() as i32 - 23 - 65;

        let win_mod: i32 = ((player_char - opponent_char + 4) % 3 - 1) * 3 + 3;
        score_sum += (player_char + 1 + win_mod) as u32;
    }

    Some(score_sum)
}

// pub fn part_two_mod(input: &str) -> Option<u32> {
//     calc_score(input, init_mappings1())
// }

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
    advent_of_code::solve!(1, part_one_mod, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input),  Some(45));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(45));
    }
}
