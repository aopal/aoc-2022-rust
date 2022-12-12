use priority_queue::DoublePriorityQueue;
use std::collections::{HashMap, HashSet};
use colored::Colorize;

fn draw_path(graph: &Vec<&str>, prev: HashMap<(usize, usize), (usize, usize)>, target: (usize, usize)) {
    let mut in_path = HashSet::new();
    let mut curr = target;
    in_path.insert(target);
    // println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());

    while prev.contains_key(&curr) {
        curr = prev[&curr];
        in_path.insert(curr);
    }

    for i in 0..graph.len() {
        for j in 0..graph[0].len() {
            let c = char_at(graph, (i,j));
            if in_path.contains(&(i,j)) {
                print!("{}", c.to_string().blue())
            } else {
                print!("{}", c)
            }
        }
        print!("\n");
    }
}

fn char_at(graph: &Vec<&str>, point: (usize, usize)) -> char {
    let mut c = graph[point.0].chars().nth(point.1).unwrap();

    if c == 'S' {
        c = 'a';
    } else if c == 'E' {
        c = 'z';
    }

    c
}

fn get_neighbours(graph: &Vec<&str>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();

    if point.0 != 0 {
        let new_point = (point.0 - 1, point.1);
        if char_at(graph, new_point) as u32 <= char_at(graph, point) as u32 + 1 {
            ret.push(new_point);
        }
    }

    if point.1 != 0 {
        let new_point = (point.0, point.1 - 1);
        if char_at(graph, new_point) as u32 <= char_at(graph, point) as u32 + 1 {
            ret.push(new_point);
        }
    }

    if point.0 != graph.len() - 1 {
        let new_point = (point.0 + 1, point.1);
        if char_at(graph, new_point) as u32 <= char_at(graph, point) as u32 + 1 {
            ret.push(new_point);
        }
    }

    if point.1 != graph[0].len() - 1 {
        let new_point = (point.0, point.1 + 1);
        if char_at(graph, new_point) as u32 <= char_at(graph, point) as u32 + 1 {
            ret.push(new_point);
        }
    }

    ret
}

fn dijkstra(graph: &Vec<&str>, source: (usize, usize)) -> (HashMap<(usize, usize), u32>, HashMap<(usize, usize), (usize, usize)>) {
    let mut pq = DoublePriorityQueue::new();
    let mut dists = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    dists.insert(source, 0 as u32);
    pq.push(source, 0 as u32);

    while pq.len() != 0 {
        let (curr, _pri) = pq.pop_min().unwrap();
        for neighbour in get_neighbours(&graph, curr) {
            let n_dist = dists[&curr] + 1;
            if !dists.contains_key(&neighbour) || n_dist < dists[&neighbour]{
                dists.insert(neighbour, n_dist);
                prev.insert(neighbour, curr);
                pq.push(neighbour, n_dist);
            }
        }
    }

    (dists, prev)
}

fn parse_input(input: &str) -> (Vec<&str>, (usize, usize), (usize, usize)) {
    let mut graph = Vec::new();
    let mut source: (usize, usize) = (0,0);
    let mut target: (usize, usize) = (0,0);
    for (i, line) in input.lines().enumerate() {
        graph.push(line);

        for (j,c) in line.chars().enumerate() {
            if c == 'S' {
                source = (i,j);
            } else if c == 'E' {
                target = (i,j);
            }
        }
    }

    (graph, source, target)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (graph, source, target) = parse_input(input);

    let (dists, prev) = dijkstra(&graph, source);

    draw_path(&graph, prev, target);

    return Some(dists[&target])
}

pub fn part_two(input: &str) -> Option<u32> {
    let (graph, source, target) = parse_input(input);

    let source_dists = dijkstra(&graph, source);

    let mut min_dist = 999999999;
    let mut min_source: (usize, usize) = (0,0);
    let mut min_prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    for i in 0..graph.len() {
        let point = (i,0);

        if char_at(&graph, point) == 'a' {
            let (dists, prev) = dijkstra(&graph, point);
            if !dists.contains_key(&target) {
                continue;
            }
            if dists[&target] < min_dist {
                min_dist = dists[&target];
                min_source = point;
                min_prev = prev;
            }
        }
    }

    draw_path(&graph, min_prev, target);

    Some(min_dist as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
