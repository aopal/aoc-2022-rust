use std::collections::{HashMap, VecDeque};
use num_traits::identities::Zero;
use num_bigint::BigInt;

#[derive(Debug)]
struct Monkey  {
    n_inspected: u32,
    items: VecDeque<BigInt>,
    operation: fn(BigInt) -> BigInt,
    throw: fn(BigInt) -> usize
}

impl Monkey {
    fn inspect(&mut self, part: u32) -> Option<(BigInt, usize)> {
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
            items: VecDeque::from([BigInt::from(79), BigInt::from(98)]),
            operation: |x: BigInt| -> BigInt { x * 19 },
            throw: |x: BigInt| -> usize {
                if ((x % 23) as BigInt).is_zero() {
                    2
                } else {
                    3
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([BigInt::from(54), BigInt::from(65), BigInt::from(75), BigInt::from(74)]),
            operation: |x: BigInt| -> BigInt { x + 6 },
            throw: |x: BigInt| -> usize {
                if ((x % 19) as BigInt).is_zero() {
                    2
                } else {
                    0
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([BigInt::from(79), BigInt::from(60), BigInt::from(97)]),
            operation: |x: BigInt| -> BigInt { x.pow(2) },
            throw: |x: BigInt| -> usize {
                if ((x % 13) as BigInt).is_zero() {
                    1
                } else {
                    3
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([BigInt::from(74)]),
            operation: |x: BigInt| -> BigInt { x + 3 },
            throw: |x: BigInt| -> usize {
                if ((x % 17) as BigInt).is_zero() {
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
            items: VecDeque::from([BigInt::from(50), BigInt::from(70), BigInt::from(54), BigInt::from(83), BigInt::from(52), BigInt::from(78)]),
            operation: |x: BigInt| -> BigInt { x * 3 },
            throw: |x: BigInt| -> usize {
                if ((x % 11) as BigInt).is_zero() {
                    2
                } else {
                    7
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([BigInt::from(71), BigInt::from(52), BigInt::from(58), BigInt::from(60), BigInt::from(71)]),
            operation: |x: BigInt| -> BigInt { x.pow(2) },
            throw: |x: BigInt| -> usize {
                if ((x % 7) as BigInt).is_zero() {
                    0
                } else {
                    2
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([BigInt::from(66), BigInt::from(56), BigInt::from(56), BigInt::from(94), BigInt::from(60), BigInt::from(86), BigInt::from(73)]),
            operation: |x: BigInt| -> BigInt { x + 1 },
            throw: |x: BigInt| -> usize {
                if ((x % 3) as BigInt).is_zero() {
                    7
                } else {
                    5
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([BigInt::from(83), BigInt::from(99)]),
            operation: |x: BigInt| -> BigInt { x + 8 },
            throw: |x: BigInt| -> usize {
                if ((x % 5) as BigInt).is_zero() {
                    6
                } else {
                    4
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([BigInt::from(98), BigInt::from(98), BigInt::from(79)]),
            operation: |x: BigInt| -> BigInt { x + 3 },
            throw: |x: BigInt| -> usize {
                if ((x % 17) as BigInt).is_zero(){
                    1
                } else {
                    0
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([BigInt::from(76)]),
            operation: |x: BigInt| -> BigInt { x + 4 },
            throw: |x: BigInt| -> usize {
                if ((x % 13) as BigInt).is_zero() {
                    6
                } else {
                    3
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([BigInt::from(52), BigInt::from(51), BigInt::from(84), BigInt::from(54)]),
            operation: |x: BigInt| -> BigInt { x * 17 },
            throw: |x: BigInt| -> usize {
                if ((x % 19) as BigInt).is_zero() {
                    4
                } else {
                    1
                }
            },
        },
        Monkey {
            n_inspected: 0,
            items: VecDeque::from([BigInt::from(82), BigInt::from(86), BigInt::from(91), BigInt::from(79), BigInt::from(94), BigInt::from(92), BigInt::from(59), BigInt::from(94)]),
            operation: |x: BigInt| -> BigInt { x + 7 },
            throw: |x: BigInt| -> usize {
                if ((x % 2) as BigInt).is_zero() {
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
