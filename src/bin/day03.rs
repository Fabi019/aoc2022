static INPUT: &str = include_str!("../../assets/day03.txt");

fn main() {
    // Part one
    let mut total_item_priority = 0;

    for line in INPUT.lines() {
        let half = line.len() / 2;

        let common = find_common(&line[..half], &line[half..]);
        total_item_priority += priority(&common);
    }

    println!("Total item priority: {}", total_item_priority);

    // Part two
    let mut total_group_priority = 0;
    let mut lines = INPUT.lines();

    while let Some(first) = lines.next() {
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        let common = find_common_three(first, second, third);
        total_group_priority += priority(&common);
    }

    println!("Total group priority: {}", total_group_priority);
}

fn find_common(left: &str, right: &str) -> char {
    for l in left.chars() {
        if right.contains(l) {
            return l;
        }
    }
    unreachable!("No common character found")
}

fn find_common_three(first: &str, second: &str, third: &str) -> char {
    for f in first.chars() {
        if second.contains(f) && third.contains(f) {
            return f;
        }
    }
    unreachable!("No common character found")
}

fn priority(item: &char) -> i32 {
    if item.is_ascii_lowercase() {
        return *item as i32 - 'a' as i32 + 1;
    } else if item.is_ascii_uppercase() {
        return *item as i32 - 'A' as i32 + 27;
    }
    unreachable!("Invalid input")
}
