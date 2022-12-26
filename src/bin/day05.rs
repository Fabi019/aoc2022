use std::collections::BTreeMap;

static INPUT: &str = include_str!("../../assets/day05.txt");

fn main() {
    let mut stacks: BTreeMap<usize, Vec<_>> = BTreeMap::new();
    let mut start_idx = 1;

    // Parse stacks
    for line in INPUT.lines() {
        if line.is_empty() {
            break;
        }

        for (idx, _) in line.match_indices('[') {
            let c = line.chars().nth(idx + 1).unwrap();
            stacks.entry(((idx + 3) / 4) + 1).or_default().push(c);
        }

        start_idx += 1;
    }

    for (_, stack) in stacks.iter_mut() {
        stack.reverse()
    }

    println!("Stacks: {:?}", stacks);

    // Move crates
    for line in INPUT.lines().skip(start_idx) {
        let mut split = line.split(' ');
        split.next(); // move
        let count = split.next().unwrap().parse::<usize>().unwrap();
        split.next(); // from
        let from = split.next().unwrap().parse::<usize>().unwrap();
        split.next(); // to
        let to = split.next().unwrap().parse::<usize>().unwrap();

        println!("Move {} from {} to {}", count, from, to);

        let mut from_stack = stacks.get(&from).unwrap().clone();
        let mut to_stack = stacks.get(&to).unwrap().clone();

        // Part one: move one at a time
        //for _ in 0..count {
        //    to_stack.push(from_stack.pop().unwrap());
        //}

        // Part two: move all at once
        to_stack.append(
            from_stack
                .drain((from_stack.len() - count)..)
                .collect::<Vec<_>>()
                .as_mut(),
        );

        stacks.insert(from, from_stack);
        stacks.insert(to, to_stack);
    }

    println!("Stacks: {:?}", stacks);

    println!(
        "Top: {}",
        stacks
            .values()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    );
}
