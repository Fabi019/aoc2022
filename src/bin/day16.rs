use std::cell::RefCell;
use std::cmp::max;
use std::collections::{BTreeSet, HashMap};
use std::hash::{Hash, Hasher};

static INPUT: &str = include_str!("../../assets/day16.txt");

#[derive(Debug)]
struct Valve {
    flow_rate: i32,
    tunnels: Vec<String>,
}

fn main() {
    let mut valves = HashMap::new();

    for line in INPUT.lines() {
        let name = line[6..8].to_string();
        let (flow, target) = line.split_once("; ").unwrap();
        let flow_rate = flow[23..].parse::<i32>().unwrap();
        let tunnels = target[22..]
            .trim()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        valves.insert(name, Valve { flow_rate, tunnels });
    }

    println!(
        "Part 1: {:?}",
        cached_dfs("AA", &BTreeSet::new(), 30, &valves)
    );
    println!(
        "Part 2: {:?}",
        cached_dfs2("AA", &BTreeSet::new(), 26, &valves)
    );
}

#[inline]
fn dfs(
    current: &str,
    opened: &BTreeSet<String>,
    time_left: i32,
    valves: &HashMap<String, Valve>,
) -> i32 {
    if time_left <= 0 || opened.len() == valves.len() {
        return 0;
    }

    let mut best = 0;
    let valve = &valves[current];

    // Check when skipping this valve
    for path in &valve.tunnels {
        best = max(best, cached_dfs(&path, opened, time_left - 1, valves));
    }

    // Check when opening this valve
    if !opened.contains(current) && valve.flow_rate > 0 {
        let flow = valve.flow_rate * (time_left - 1);

        let mut opened = opened.clone();
        opened.insert(current.to_string());

        for path in &valve.tunnels {
            let next = cached_dfs(&path, &opened, time_left - 2, valves);
            best = max(best, flow + next);
        }
    }

    best
}

#[inline]
fn dfs2(
    current: &str,
    opened: &BTreeSet<String>,
    time_left: i32,
    valves: &HashMap<String, Valve>,
) -> i32 {
    if time_left <= 0 || opened.len() == valves.len() {
        // Simulate the elephant
        return cached_dfs("AA", opened, 26, valves);
    }

    let mut best = 0;
    let valve = &valves[current];

    // Check when skipping this valve
    for path in &valve.tunnels {
        best = max(best, cached_dfs2(&path, opened, time_left - 1, valves));
    }

    // Check when opening this valve
    if !opened.contains(current) && valve.flow_rate > 0 {
        let flow = valve.flow_rate * (time_left - 1);

        let mut opened = opened.clone();
        opened.insert(current.to_string());

        for path in &valve.tunnels {
            let next = cached_dfs2(&path, &opened, time_left - 2, valves);
            best = max(best, flow + next);
        }
    }

    best
}

// Memoization cache
std::thread_local! {
    static MEMOIZED_MAPPING_DFS : RefCell<HashMap<u64, i32>> = RefCell::new(HashMap::new());
    static MEMOIZED_MAPPING_DFS_2 : RefCell<HashMap<u64, i32>> = RefCell::new(HashMap::new());
}

fn cached_dfs(
    current: &str,
    opened: &BTreeSet<String>,
    time_left: i32,
    valves: &HashMap<String, Valve>,
) -> i32 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    current.hash(&mut hasher);
    time_left.hash(&mut hasher);
    for v in opened {
        v.hash(&mut hasher);
    }

    let key = hasher.finish();

    if let Some(r) = MEMOIZED_MAPPING_DFS.with(|m| m.borrow().get(&key).cloned()) {
        return r;
    }

    let r = dfs(current, opened, time_left, valves);

    MEMOIZED_MAPPING_DFS.with(|m| {
        m.borrow_mut().insert(key, r);
    });

    r
}

fn cached_dfs2(
    current: &str,
    opened: &BTreeSet<String>,
    time_left: i32,
    valves: &HashMap<String, Valve>,
) -> i32 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    current.hash(&mut hasher);
    time_left.hash(&mut hasher);
    for v in opened {
        v.hash(&mut hasher);
    }

    let key = hasher.finish();

    if let Some(r) = MEMOIZED_MAPPING_DFS_2.with(|m| m.borrow().get(&key).cloned()) {
        return r;
    }

    let r = dfs2(current, opened, time_left, valves);

    MEMOIZED_MAPPING_DFS_2.with(|m| {
        m.borrow_mut().insert(key, r);
    });

    r
}
