use std::collections::HashMap;

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
    let y = 2000000;

    let mut ranges = Vec::new();

    // Collect all ranges of sensors at y-coordinate
    for ((sx, sy), Sensor(_, dst)) in map {
        let dy = sy.abs_diff(y) as i32;
        if dy <= *dst {
            let offset = dst.abs_diff(dy) as i32;
            let (start, end) = (sx - offset, sx + offset);
            ranges.push((start, end));
        }
    }

    ranges.sort_unstable_by_key(|(start, _)| *start);

    // Merge overlapping ranges
    let result = ranges.iter().fold(Vec::new(), |mut res, r @ (_, hi)| {
        match res.last() {
            Some((last_lo, last_hi)) if last_lo < hi => {
                let len = res.len();
                res[len - 1] = (*last_lo, *hi.max(last_hi));
            }
            None => res.push(*r),
            _ => (),
        }
        res
    });

    // Sum the lengths of all the ranges
    let count = result
        .iter()
        .map(|(start, end)| start.abs_diff(*end))
        .sum::<u32>();

    println!("Part 1: {}", count);
}

fn part2(map: &HashMap<(i32, i32), Sensor>) {
    let max_x: i32 = 4000000;
    let max_y: i32 = 4000000;

    for y in 0..=max_y {
        let mut x = 0;

        while x <= max_x {
            let current = (x, y);

            // Find next sensor in range
            let sensor = map
                .iter()
                .find(|(s, Sensor(_, dst))| manhatten_distance(s, &current) <= *dst);

            // Calculate next x-coordinate
            if let Some(((sx, sy), Sensor(_, dst))) = sensor {
                let dy = sy.abs_diff(y) as i32;
                let offset = dst.abs_diff(dy) as i32;
                x = sx + offset + 1;
            } else {
                println!("Part 2: {}", (x as isize) * 4000000 + y as isize);
                return;
            }
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
