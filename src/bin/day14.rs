use std::collections::HashMap;

static INPUT: &str = include_str!("../../assets/day14.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Material {
    Air,
    Rock,
    Sand,
    StillSand,
}

#[allow(unused_labels)]
fn main() {
    let mut cave: HashMap<(isize, isize), Material> = HashMap::new();

    for line in INPUT.lines() {
        let mut prev = None;
        for coord in line.split(" -> ") {
            let (x, y) = coord.split_once(',').unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            if let Some((prev_x, prev_y)) = prev {
                let (min_x, max_x) = if x < prev_x { (x, prev_x) } else { (prev_x, x) };
                let (min_y, max_y) = if y < prev_y { (y, prev_y) } else { (prev_y, y) };
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        cave.insert((x, y), Material::Rock);
                    }
                }
            }
            prev = Some((x, y));
        }
    }

    let max_y = *cave.keys().map(|(_, y)| y).max().unwrap();

    'outer: for i in 1.. {
        println!("\n=== Sand {} ===\n", i);

        let mut current = (500, 0);
        cave.insert(current, Material::Sand);

        while let Some(next @ (_, y)) = next_pos(&cave, current) {
            // Part 1
            /* if y > max_y {
                // Sand would fall out of bounds
                break 'outer;
            } */

            cave.insert(current, Material::Air);
            cave.insert(next, Material::Sand);
            current = next;

            // Part 2
            if y == max_y + 1 {
                // Minimum point reached
                break;
            }
        }

        cave.insert(current, Material::StillSand);
        //print_cave(&cave);

        if current == (500, 0) {
            // No more sand to fall
            break;
        }
    }

    let sand_count = cave.values().filter(|m| **m == Material::StillSand).count();
    println!("Sand count: {}", sand_count);
}

fn next_pos(
    cave: &HashMap<(isize, isize), Material>,
    (x, y): (isize, isize),
) -> Option<(isize, isize)> {
    for (dx, dy) in &[(0, 1), (-1, 1), (1, 1)] {
        let (tx, ty) = (x + dx, y + dy);
        match cave.get(&(tx, ty)) {
            Some(Material::Air) | None => return Some((tx, ty)),
            _ => continue,
        }
    }
    None
}

#[allow(dead_code)]
fn print_cave(cave: &HashMap<(isize, isize), Material>) {
    let min_x = *cave.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *cave.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *cave.keys().map(|(_, y)| y).max().unwrap();

    for y in 0..=max_y {
        for x in min_x..=max_x {
            match cave.get(&(x, y)) {
                Some(Material::Rock) => print!("#"),
                Some(Material::Sand) => print!("+"),
                Some(Material::StillSand) => print!("o"),
                _ => print!("."),
            }
        }
        println!();
    }
}
