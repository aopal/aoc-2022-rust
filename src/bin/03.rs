pub fn part_one(input: &str) -> Option<u32> {
    let mut pri_sum: u32 = 0;

    for line in input.lines() {
        let mut char_map: [u32; 53] = [0; 53];
        let line_len = line.len();

        for (i, char) in line.chars().enumerate() {
            let pri = char_to_priority(char);

            if i < line_len / 2 {
                char_map[pri as usize] += 1;
            } else {
                if char_map[pri as usize] != 0 {
                    pri_sum += pri;
                    break;
                }
            }
        }
    }

    Some(pri_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut pri_sum: u32 = 0;
    let mut lines_iter = input.lines();

    loop {
        let line1 = lines_iter.next();
        let line2 = lines_iter.next();
        let line3 = lines_iter.next();

        if line1.is_none() || line2.is_none() || line3.is_none() {
            break;
        }

        let l1 = String::from(line1.unwrap());
        let l2 = String::from(line2.unwrap());
        let l3 = String::from(line3.unwrap());

        let mut char_map1: [u32; 53] = [0; 53];
        let mut char_map2: [u32; 53] = [0; 53];
        let mut char_map3: [u32; 53] = [0; 53];

        count_chars(&l1, &mut char_map1);
        count_chars(&l2, &mut char_map2);
        count_chars(&l3, &mut char_map3);

        let mut added_to_sum = false;
        for i in (1..53) {
            if char_map1[i] == 1 && char_map2[i] == 1 && char_map3[i] == 1 {
                pri_sum += i as u32;
                added_to_sum = true
            }
        }

        if !added_to_sum {
            println!("shouldn't reach here: {} {} {}", l1, l2, l3);
        }
    }

    Some(pri_sum)
}

fn count_chars(line: &String, char_map: &mut [u32; 53]) {
    for char in line.chars() {
        let pri = char_to_priority(char);
        // println!("{} {}", char, pri);
        char_map[pri as usize] = 1;
    }
}

fn char_to_priority(c: char) -> u32 {
    let mut i = c as u32;

    if i >= 65 && i <= 90 {
        // A to Z
        i -= 38
    } else if i >= 97 && i <= 122 {
        // a to z
        i -= 96
    }

    i
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
