use std::collections::{HashMap, VecDeque};
use num_traits::identities::Zero;

#[derive(Debug)]
struct Monkey  {
    n_inspected: u32,
    items: VecDeque<i64>,
    operation: fn(i64) -> i64,
    throw: fn(i64) -> usize
}

impl Monkey {
    fn inspect(&mut self, part: u32) -> Option<(i64, usize)> {
        let mut item = self.items.pop_front();
        if item.is_none() {
            return None;
        }
        self.n_inspected += 1;

        let mut i = item.unwrap();
        i = (self.operation)(i);
        if part == 1 {
            i = i/3;
        } else if part == 2 {
            i = i % 9699690;
            // i = i % 96577 // for sample input
        }

        let idx = (self.throw)(i.clone());

        Some((i, idx))
    }
}


fn init_sample() -> Vec<Monkey>  {
    vec![
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([i64::from(79), i64::from(98)]),
            operation: |x: i64| -> i64 { x * 19 },
            throw: |x: i64| -> usize {
                if ((x % 23) as i64).is_zero() {
                    2
                } else {
                    3
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([i64::from(54), i64::from(65), i64::from(75), i64::from(74)]),
            operation: |x: i64| -> i64 { x + 6 },
            throw: |x: i64| -> usize {
                if ((x % 19) as i64).is_zero() {
                    2
                } else {
                    0
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([i64::from(79), i64::from(60), i64::from(97)]),
            operation: |x: i64| -> i64 { x.pow(2) },
            throw: |x: i64| -> usize {
                if ((x % 13) as i64).is_zero() {
                    1
                } else {
                    3
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([i64::from(74)]),
            operation: |x: i64| -> i64 { x + 3 },
            throw: |x: i64| -> usize {
                if ((x % 17) as i64).is_zero() {
                    0
                } else {
                    1
                }
            },
        },
    ]
}

fn init() -> Vec<Monkey> {
    let v: Vec<Monkey>;

    v = vec![
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([i64::from(50), i64::from(70), i64::from(54), i64::from(83), i64::from(52), i64::from(78)]),
            operation: |x: i64| -> i64 { x * 3 },
            throw: |x: i64| -> usize {
                if ((x % 11) as i64).is_zero() {
                    2
                } else {
                    7
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([i64::from(71), i64::from(52), i64::from(58), i64::from(60), i64::from(71)]),
            operation: |x: i64| -> i64 { x.pow(2) },
            throw: |x: i64| -> usize {
                if ((x % 7) as i64).is_zero() {
                    0
                } else {
                    2
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([i64::from(66), i64::from(56), i64::from(56), i64::from(94), i64::from(60), i64::from(86), i64::from(73)]),
            operation: |x: i64| -> i64 { x + 1 },
            throw: |x: i64| -> usize {
                if ((x % 3) as i64).is_zero() {
                    7
                } else {
                    5
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([i64::from(83), i64::from(99)]),
            operation: |x: i64| -> i64 { x + 8 },
            throw: |x: i64| -> usize {
                if ((x % 5) as i64).is_zero() {
                    6
                } else {
                    4
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([i64::from(98), i64::from(98), i64::from(79)]),
            operation: |x: i64| -> i64 { x + 3 },
            throw: |x: i64| -> usize {
                if ((x % 17) as i64).is_zero(){
                    1
                } else {
                    0
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([i64::from(76)]),
            operation: |x: i64| -> i64 { x + 4 },
            throw: |x: i64| -> usize {
                if ((x % 13) as i64).is_zero() {
                    6
                } else {
                    3
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([i64::from(52), i64::from(51), i64::from(84), i64::from(54)]),
            operation: |x: i64| -> i64 { x * 17 },
            throw: |x: i64| -> usize {
                if ((x % 19) as i64).is_zero() {
                    4
                } else {
                    1
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([i64::from(82), i64::from(86), i64::from(91), i64::from(79), i64::from(94), i64::from(92), i64::from(59), i64::from(94)]),
            operation: |x: i64| -> i64 { x + 7 },
            throw: |x: i64| -> usize {
                if ((x % 2) as i64).is_zero() {
                    5
                } else {
                    3
                }
            },
        },
    ];


    v
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys = init();
    for k in 0..20 {
        for i in 0..monkeys.len() {
            loop {
                let throw_to = monkeys[i].inspect(1);
                if throw_to.is_none() {
                    break;
                }

                let (n, idx) = throw_to.unwrap();
                monkeys[idx].items.push_back(n);
            }
        }

        // println!("\n\nround {}", k);
        // for i in 0..monkeys.len() {
        //     println!("{:?}", monkeys[i]);
        // }
    }

    for i in 0..monkeys.len() {
        println!("{:?}", monkeys[i]);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut monkeys = init();
    for k in 0..10000 {
        for i in 0..monkeys.len() {
            loop {
                let throw_to = monkeys[i].inspect(2);
                if throw_to.is_none() {
                    break;
                }

                let (n, idx) = throw_to.unwrap();
                monkeys[idx].items.push_back(n);
            }
        }

        // println!("\n\nround {}", k);
        // for i in 0..monkeys.len() {
        //     println!("{:?}", monkeys[i].n_inspected);
        // }
    }
    for i in 0..monkeys.len() {
        println!("{:?}", monkeys[i]);
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
