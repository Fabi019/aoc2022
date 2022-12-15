use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../../assets/day15.txt");

// Sensor with beacon coords and precalculated distance
struct Sensor((i32, i32), i32);

fn main() {
    let mut map = HashMap::new();

    for line in INPUT.lines() {
        let (sensor, beacon) = line.split_once(": ").unwrap();
        let sensor_coords = parse_coords(&sensor[10..]);
        let beacon_coords = parse_coords(&beacon[21..]);
        let distance = manhatten_distance(&sensor_coords, &beacon_coords);
        map.insert(sensor_coords, Sensor(beacon_coords, distance));
    }

    part1(&map);
    part2(&map);
}

fn part1(map: &HashMap<(i32, i32), Sensor>) {
    let target_y = 2000000;
    let mut not_map = HashSet::new();

    for (pos @ (x, y), Sensor(beacon, distance)) in map.iter() {
        println!("S: {:?} -> {:?}; d = {}", pos, beacon, distance);

        if y + distance < target_y {
            continue;
        }

        let y = target_y;
        for x in x - distance..=x + distance {
            let current = (x, y);
            if &current == beacon {
                continue;
            }
            if manhatten_distance(pos, &current) <= *distance {
                not_map.insert(current);
            }
        }
    }

    println!("Part 1: {}", not_map.len());
}

fn part2(map: &HashMap<(i32, i32), Sensor>) {
    let max_x: i32 = 4000000;
    let max_y: i32 = 4000000;

    let mut ranges: Vec<(i32, i32)> = Vec::new();

    for y in 0..=max_y {
        ranges.clear();

        // Calculate the range span for each sensor on this line
        for ((sx, sy), Sensor(_, distance)) in map {
            let dy = sy.abs_diff(y) as i32;
            if dy <= *distance {
                let offset = distance.abs_diff(dy) as i32;
                ranges.push((sx - offset, sx + offset));
            }
        }

        // Sort the ranges by x coordinate
        ranges.sort_unstable_by_key(|(x, _)| *x);

        // Combine overlapping ranges
        let mut combined: Vec<(i32, i32)> = Vec::new();
        let mut current_end = &ranges[0].1;

        for (start, end) in &ranges {
            combined.retain(|(s, e)| s > start || e < end);

            if *start > current_end + 1 && *end < max_x - 1 {
                combined.push((current_end + 1, start - 1));
            }

            current_end = end.max(current_end);
        }

        // Check if there are any ranges left
        if combined.len() > 0 {
            for (l, _) in combined {
                println!("Part 2: {}", (l as isize) * 4000000 + y as isize);
            }
            break;
        }
    }
}

fn manhatten_distance((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> i32 {
    (x2.abs_diff(*x1) + y2.abs_diff(*y1)) as i32
}

fn parse_coords(coords: &str) -> (i32, i32) {
    let (x, y) = coords.split_once(", ").unwrap();
    (x[2..].parse().unwrap(), y[2..].parse().unwrap())
}
