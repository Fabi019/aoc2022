use std::{
    cell::RefCell,
    cmp::max,
    collections::{HashMap, VecDeque},
    hash::{Hash, Hasher},
    str::FromStr,
};

static INPUT: &str = include_str!("../../assets/day19.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blueprint {
    ore_robot_cost: i32,
    clay_robot_cost: i32,
    obsidian_robot_cost: (i32, i32),
    geode_robot_cost: (i32, i32),
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        let ore_robot_cost = split.nth(6).unwrap().parse::<i32>().unwrap();
        let clay_robot_cost = split.nth(5).unwrap().parse::<i32>().unwrap();
        let obsidian_robot_cost = (
            split.nth(5).unwrap().parse::<i32>().unwrap(),
            split.nth(2).unwrap().parse::<i32>().unwrap(),
        );
        let geode_robot_cost = (
            split.nth(5).unwrap().parse::<i32>().unwrap(),
            split.nth(2).unwrap().parse::<i32>().unwrap(),
        );

        Ok(Self {
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        })
    }
}

fn main() {
    let blueprints = INPUT
        .lines()
        .map(|l| l.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    let sum = blueprints
        .iter()
        .enumerate()
        .map(|(idx, b)| (idx + 1) as i32 * geodes_iterativ(b, 24))
        .sum::<i32>();

    println!("Part 1: {}", sum);

    let product = blueprints
        .iter()
        .take(3)
        .map(|b| geodes_iterativ(b, 32))
        .product::<i32>();

    println!("Part 2: {}", product);
}

fn geodes_iterativ(blueprint: &Blueprint, minutes: i32) -> i32 {
    let ore_cost = blueprint.ore_robot_cost;
    let clay_cost = blueprint.clay_robot_cost;
    let (obs_ore_cost, obs_clay_cost) = blueprint.obsidian_robot_cost;
    let (geo_ore_cost, geo_obs_cost) = blueprint.geode_robot_cost;

    let max_ore = max(max(max(ore_cost, clay_cost), obs_ore_cost), geo_ore_cost);

    let mut queue = VecDeque::new();
    queue.push_back((minutes, 1, 0, 0, 0, 0, 0, 0, 0));

    let mut cache = HashMap::new();

    let mut max_geodes = 0;

    while let Some((
        minutes,
        ore_robots,
        clay_robots,
        obsidian_robots,
        geode_robots,
        ores,
        clays,
        obsidians,
        geodes,
    )) = queue.pop_front()
    {
        let cache_entry = cache
            .entry((
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots,
                ores,
                clays,
                obsidians,
                geodes,
            ))
            .or_insert(-1);
        if *cache_entry >= minutes {
            continue;
        } else {
            *cache_entry = minutes;
        }

        if minutes <= 1 {
            max_geodes = max(max_geodes, geodes + geode_robots);
            continue;
        }

        if ores >= geo_ore_cost && obsidians >= geo_obs_cost {
            queue.push_back((
                minutes - 1,
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots + 1,
                ores - geo_ore_cost + ore_robots,
                clays + clay_robots,
                obsidians - geo_obs_cost + obsidian_robots,
                geodes + geode_robots,
            ));
            continue;
        }

        if ores >= obs_ore_cost && clays >= obs_clay_cost && obsidian_robots < geo_obs_cost {
            queue.push_back((
                minutes - 1,
                ore_robots,
                clay_robots,
                obsidian_robots + 1,
                geode_robots,
                ores - obs_ore_cost + ore_robots,
                clays - obs_clay_cost + clay_robots,
                obsidians + obsidian_robots,
                geodes + geode_robots,
            ));
        }

        if ores >= clay_cost && clay_robots < obs_clay_cost {
            queue.push_back((
                minutes - 1,
                ore_robots,
                clay_robots + 1,
                obsidian_robots,
                geode_robots,
                ores - clay_cost + ore_robots,
                clays + clay_robots,
                obsidians + obsidian_robots,
                geodes + geode_robots,
            ));
        }

        if ores >= ore_cost && ore_robots < max_ore {
            queue.push_back((
                minutes - 1,
                ore_robots + 1,
                clay_robots,
                obsidian_robots,
                geode_robots,
                ores - ore_cost + ore_robots,
                clays + clay_robots,
                obsidians + obsidian_robots,
                geodes + geode_robots,
            ));
        }

        // Do nothing
        queue.push_back((
            minutes - 1,
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots,
            ores + ore_robots,
            clays + clay_robots,
            obsidians + obsidian_robots,
            geodes + geode_robots,
        ));
    }

    max_geodes
}

/**
 * DOES NOT WORK!
 */

#[inline]
fn geodes_recursive(
    blueprint: &Blueprint,
    ores: i32,
    clays: i32,
    obsidians: i32,
    geodes: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    minutes: i32,
) -> i32 {
    if minutes <= 1 {
        return geodes + geode_robots;
    }

    let (obsidian_ore_cost, obsidian_clay_cost) = blueprint.obsidian_robot_cost;
    let (geode_ore_cost, geode_obsidian_cost) = blueprint.geode_robot_cost;

    let max_ore = max(
        max(
            max(blueprint.ore_robot_cost, blueprint.clay_robot_cost),
            obsidian_ore_cost,
        ),
        geode_ore_cost,
    );

    let mut max_geodes = 0;

    // Buy geode robot
    if ores >= geode_ore_cost && obsidians >= geode_obsidian_cost {
        return cached_geodes(
            blueprint,
            ores + ore_robots - geode_ore_cost,
            clays + clay_robots,
            obsidians + obsidians - geode_obsidian_cost,
            geodes + geode_robots,
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots + 1,
            minutes - 1,
        );
    }

    // Buy ore robot
    if ores >= blueprint.ore_robot_cost && ore_robots < max_ore {
        max_geodes = max(
            max_geodes,
            cached_geodes(
                blueprint,
                ores + ore_robots - blueprint.ore_robot_cost,
                clays + clay_robots,
                obsidians + obsidian_robots,
                geodes + geode_robots,
                ore_robots + 1,
                clay_robots,
                obsidian_robots,
                geode_robots,
                minutes - 1,
            ),
        );
    }

    // Buy clay robot
    if ores >= blueprint.clay_robot_cost && clay_robots < obsidian_clay_cost {
        max_geodes = max(
            max_geodes,
            cached_geodes(
                blueprint,
                ores + ore_robots - blueprint.clay_robot_cost,
                clays + clay_robots,
                obsidians + obsidian_robots,
                geodes + geode_robots,
                ore_robots,
                clay_robots + 1,
                obsidian_robots,
                geode_robots,
                minutes - 1,
            ),
        );
    }

    // Buy obsidian robot
    if ores >= obsidian_ore_cost
        && clays >= obsidian_clay_cost
        && obsidian_robots < geode_obsidian_cost
    {
        max_geodes = max(
            max_geodes,
            cached_geodes(
                blueprint,
                ores + ore_robots - obsidian_ore_cost,
                clays + clay_robots - obsidian_clay_cost,
                obsidians + obsidian_robots,
                geodes + geode_robots,
                ore_robots,
                clay_robots,
                obsidian_robots + 1,
                geode_robots,
                minutes - 1,
            ),
        );
    }

    // Buy nothing
    max_geodes = max(
        max_geodes,
        cached_geodes(
            blueprint,
            ores + ore_robots,
            clays + clay_robots,
            obsidians + obsidian_robots,
            geodes + geode_robots,
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots,
            minutes - 1,
        ),
    );

    max_geodes
}

// Memoization cache
std::thread_local! {
    static MEMOIZED_MAPPING_GEODES : RefCell<HashMap<u64, i32>> = RefCell::new(HashMap::new());
}

fn clear_memoized_mapping_geodes() {
    MEMOIZED_MAPPING_GEODES.with(|m| m.borrow_mut().clear());
}

fn cached_geodes(
    blueprint: &Blueprint,
    ores: i32,
    clays: i32,
    obsidians: i32,
    geodes: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    minutes: i32,
) -> i32 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    ores.hash(&mut hasher);
    clays.hash(&mut hasher);
    obsidians.hash(&mut hasher);
    geodes.hash(&mut hasher);
    ore_robots.hash(&mut hasher);
    clay_robots.hash(&mut hasher);
    obsidian_robots.hash(&mut hasher);
    geode_robots.hash(&mut hasher);
    minutes.hash(&mut hasher);

    let key = hasher.finish();

    MEMOIZED_MAPPING_GEODES.with(|m| {
        if let Some(r) = m.borrow().get(&key) {
            //println!("Cache hit: {:?}", key);
            return *r;
        }
        let r = geodes_recursive(
            blueprint,
            ores,
            clays,
            obsidians,
            geodes,
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots,
            minutes,
        );
        m.borrow_mut().insert(key, r);
        r
    })
}
