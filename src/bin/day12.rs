use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

static INPUT: &str = include_str!("../../assets/day12.txt");

fn main() {
    let mut height_map = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Find S and E positions in height_map
    let mut start = (0, 0);
    let mut dest = (0, 0);

    let mut starts = Vec::new();

    for (y, row) in height_map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            print!("{}", c as char);
            match c as char {
                'S' => start = (x, y),
                'E' => dest = (x, y),
                'a' => starts.push((x, y)),
                _ => (),
            }
        }
        println!();
    }

    // Replace S and E with a and z
    height_map[start.1][start.0] = 'a' as u8;
    height_map[dest.1][dest.0] = 'z' as u8;

    let path = dijkstra(&height_map, start, dest).unwrap();

    visualize_path(&height_map, &path);

    let mut minimum_lenght = path.len() - 1;

    for start in starts {
        if let Some(path) = dijkstra(&height_map, start, dest) {
            minimum_lenght = minimum_lenght.min(path.len() - 1);
        }
    }

    println!("Part 1: {}", path.len() - 1);
    println!("Part 2: {}", minimum_lenght);
}

fn dijkstra(
    height_map: &[Vec<u8>],
    start: (usize, usize),
    dest: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut prev = HashMap::new();
    let mut costs = HashMap::new();

    queue.push(Reverse((0, start)));
    costs.insert(start, 0);

    while let Some(Reverse((cost, pos))) = queue.pop() {
        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        if pos == dest {
            break;
        }

        for neighbour in neighbours(height_map, pos) {
            if visited.contains(&neighbour) {
                continue;
            }

            let new_cost = cost + 1;

            let cur_cost = costs.get(&neighbour);
            if cur_cost.is_none() || new_cost < *cur_cost.unwrap() {
                costs.insert(neighbour, new_cost);
                prev.insert(neighbour, pos);
            }

            queue.push(Reverse((new_cost, neighbour)));
        }
    }

    let mut path = vec![dest];
    let mut cur_pos = dest;

    while cur_pos != start {
        if !prev.contains_key(&cur_pos) {
            // Path is not complete
            return None;
        }
        cur_pos = prev[&cur_pos];
        path.push(cur_pos);
    }

    path.reverse();
    Some(path)
}

fn neighbours(height_map: &[Vec<u8>], pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    let (x, y) = pos;
    let current = height_map[y][x] as i32;

    if x > 0 {
        let target = height_map[y][x - 1] as i32;
        if target - current <= 1 {
            neighbours.push((x - 1, y));
        }
    }

    if x < height_map[0].len() - 1 {
        let target = height_map[y][x + 1] as i32;
        if target - current <= 1 {
            neighbours.push((x + 1, y));
        }
    }

    if y > 0 {
        let target = height_map[y - 1][x] as i32;
        if target - current <= 1 {
            neighbours.push((x, y - 1));
        }
    }

    if y < height_map.len() - 1 {
        let target = height_map[y + 1][x] as i32;
        if target - current <= 1 {
            neighbours.push((x, y + 1));
        }
    }

    neighbours
}

fn visualize_path(height_map: &[Vec<u8>], path: &[(usize, usize)]) {
    let mut path_map = vec![vec!['.'; height_map[0].len()]; height_map.len()];
    for (i, pos) in path.iter().enumerate() {
        if i == path.len() - 1 {
            break;
        }

        let next_pos = path[i + 1];

        if next_pos.0 > pos.0 {
            path_map[pos.1][pos.0] = '>';
        } else if next_pos.0 < pos.0 {
            path_map[pos.1][pos.0] = '<';
        } else if next_pos.1 > pos.1 {
            path_map[pos.1][pos.0] = 'v';
        } else if next_pos.1 < pos.1 {
            path_map[pos.1][pos.0] = '^';
        }
    }

    for row in path_map {
        for c in row {
            print!("{}", c as char);
        }
        println!();
    }
}
