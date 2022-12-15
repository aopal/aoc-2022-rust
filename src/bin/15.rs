use regex::Regex;
use std::collections::HashSet;
use std::cmp;
use std::ops::RangeInclusive;


#[derive(Debug)]
struct MultiRange {

}

#[derive(Debug)]
struct Sensor {
    point: (i32, i32),
    closest_beacon: (i32, i32),
}

impl Sensor {
    fn radius(&self) -> i32 {
        // (self.point.0 - self.closest_beacon.0).abs() + (self.point.1 - self.closest_beacon.1).abs()
        self.dist(self.closest_beacon)
    }

    fn dist(&self, point: (i32, i32)) -> i32 {
        (self.point.0 - point.0).abs() + (self.point.1 - point.1).abs()
    }

    fn x(&self) -> i32 { self.point.0 }
    fn y(&self) -> i32 { self.point.1 }
}

fn build_sensors(input: &str) -> Vec<Sensor> {
    let mut v = Vec::<Sensor>::new();
    let sensor_re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    for line in input.lines() {
        let (mut sx,mut sy,mut bx,mut by) = (0,0,0,0);

        let captures = sensor_re.captures(line).unwrap();
        sx = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        sy = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        bx = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        by = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();

        v.push(Sensor {
            point: (sx, sy),
            closest_beacon: (bx, by),
        })
    }

    v
}

fn count_not_possible(sensors: Vec<Sensor>, y: i32) -> Option<u32> {
    // let sensors = build_sensors(input);
    // let y = 10;
    // let y = 2000000;
    let mut points = HashSet::new();
    let mut beacons = HashSet::new();
    let mut ranges: Vec<RangeInclusive<i32>> = Vec::new();

    for sensor in sensors {
        if sensor.closest_beacon.1 == y {
            beacons.insert(sensor.closest_beacon.0);
        }

        let del_y = (y-sensor.y()).abs();
        if del_y > sensor.radius() {
            continue;
        }

        let del_x = sensor.radius() - del_y;
        let r = (sensor.x() - del_x)..=(sensor.x() + del_x);
        println!("{:?}", r);
        for x in r.clone() {
            points.insert(x);
        }

        for range in ranges.iter() {

        }
        ranges.push(r);
    }

    Some((points.len() - beacons.len()) as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    count_not_possible(build_sensors(input), 2000000)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sensors = build_sensors(input);
    let mut point = (-1,-1);
    // let upper = 20;
    let upper = 4000000;

    for y in 0..=upper {
        if y % 10000 == 0 {
            println!("y: {}", y);
        }

        let mut x = 0;
        while x <= upper {
            x += 1;
            let mut in_radius = false;
            for sensor in sensors.iter() {
                let s_dist = sensor.dist((x,y));
                if s_dist <= sensor.radius() {
                    in_radius = true;
                    let y_dist = (sensor.y() - y).abs();
                    x = sensor.x() + (sensor.radius() - y_dist);
                    break;
                }
            }
            if !in_radius {
                point = (x,y);
                println!("point: {:?}", point);
                return Some((point.0 * 4000000 + point.1) as u32)
            }
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
