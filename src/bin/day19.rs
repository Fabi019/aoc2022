use std::{
    cmp::max,
    collections::{HashMap, VecDeque},
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
