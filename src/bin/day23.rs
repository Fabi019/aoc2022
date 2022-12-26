use std::collections::{HashMap, HashSet, VecDeque};

static INPUT: &str = include_str!("../../assets/day23.txt");

fn main() {
    let mut elves = INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Some((x as i32, y as i32)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();

    // Directions for the elves to move in
    let mut directions = VecDeque::new();
    // N, NE, or NW adjacent positions
    directions.push_back([(0, -1), (1, -1), (-1, -1)]);
    // S, SE, or SW adjacent positions
    directions.push_back([(0, 1), (1, 1), (-1, 1)]);
    // W, NW, or SW adjacent positions
    directions.push_back([(-1, 0), (-1, -1), (-1, 1)]);
    // E, NE, or SE adjacent positions
    directions.push_back([(1, 0), (1, -1), (1, 1)]);

    for round in 1.. {
        let mut target_positions = HashMap::new();
        let mut not_moving = 0;

        // Find the target positions for each elf
        for elf @ (x, y) in &elves {
            // If no other Elves are in one of those eight positions the Elf does not do anything during this round
            if directions
                .iter()
                .flat_map(|offset| offset.iter())
                .all(|(dx, dy)| !elves.contains(&(x + dx, y + dy)))
            {
                not_moving += 1;
                continue;
            }

            'direction: for offset in &directions {
                // Check if other elves are in the way
                for (dx, dy) in offset {
                    let target = (x + dx, y + dy);
                    if elves.contains(&target) {
                        continue 'direction;
                    }
                }
                let (dx, dy) = offset[0];
                target_positions.insert(*elf, (x + dx, y + dy));
                break;
            }
        }

        if not_moving == elves.len() {
            println!("Part 2: {}", round);
            break;
        }

        // First direction is moved to the end of the list of directions
        let first = directions.pop_front().unwrap();
        directions.push_back(first);

        let targets = target_positions.clone();

        // Check if there are duplicate target positions
        for target in targets.values() {
            let duplicates = targets
                .iter()
                .filter(|(_, t)| t == &target)
                .collect::<Vec<_>>();
            if duplicates.len() > 1 {
                // Remove the duplicates
                for (e, _) in duplicates {
                    target_positions.remove(e);
                }
            }
        }

        // Move the elves
        for (elf, target) in target_positions {
            elves.remove(&elf);
            elves.insert(target);
        }

        // Check if this is the 10th round
        if round == 10 {
            let ground_count = count_ground(&elves);
            println!("Part 1: {}", ground_count);
        }
    }
}

fn count_ground(elves: &HashSet<(i32, i32)>) -> i32 {
    // Calculate Minimum Bounding Rectangle for the elves
    let min_x = elves.iter().map(|(x, _)| x).min().unwrap();
    let max_x = elves.iter().map(|(x, _)| x).max().unwrap();
    let min_y = elves.iter().map(|(_, y)| y).min().unwrap();
    let max_y = elves.iter().map(|(_, y)| y).max().unwrap();

    let mut ground_count = 0;

    // Count the number of ground tiles
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            if !elves.contains(&(x, y)) {
                ground_count += 1;
            }
        }
    }

    ground_count
}
