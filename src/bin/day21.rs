use std::{collections::HashMap, cmp::Ordering};

static INPUT: &str = include_str!("../../assets/day21.txt");

#[derive(Debug, Clone)]
enum Job {
    Number(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn main() {
    let jobs = INPUT
        .lines()
        .map(|line| {
            let (name, job) = line.split_once(": ").unwrap();

            let mut job = job.split_whitespace();

            let job = match (job.next(), job.next(), job.next()) {
                (Some(a), Some("+"), Some(b)) => Job::Add(a.to_string(), b.to_string()),
                (Some(a), Some("-"), Some(b)) => Job::Sub(a.to_string(), b.to_string()),
                (Some(a), Some("*"), Some(b)) => Job::Mul(a.to_string(), b.to_string()),
                (Some(a), Some("/"), Some(b)) => Job::Div(a.to_string(), b.to_string()),
                (Some(a), None, None) => Job::Number(a.parse().unwrap()),
                _ => panic!("Unknown job: {}", line),
            };

            (name.to_string(), job)
        })
        .collect::<HashMap<_, _>>();

    println!("Part 1: {:?}", get_value(&jobs, "root"));

    // Get the first and second operand of root
    let (first, second) = match &jobs["root"] {
        Job::Add(a, b) => (a, b),
        Job::Sub(a, b) => (a, b),
        Job::Mul(a, b) => (a, b),
        Job::Div(a, b) => (a, b),
        _ => panic!("Root is not an operation"),
    };

    // Always stays the same
    let b = get_value(&jobs, second);

    let mut jobs = jobs.clone();

    let mut min = 0;
    let mut max = 100000000000000;

    while min != max {
        let mid = max - (max - min) / 2;

        jobs.insert("humn".to_string(), Job::Number(mid));
        let a = get_value(&jobs, first);

        println!("{} {} {} {}", min, mid, max, a - b);

        match a.cmp(&b) {
            Ordering::Less => max = mid - 1,
            Ordering::Equal => break,
            Ordering::Greater => min = mid + 1,
        }
    }

    println!("Part 2: {}", min);
}

fn get_value(jobs: &HashMap<String, Job>, value: &str) -> i64 {
    match jobs.get(value).unwrap() {
        Job::Number(n) => *n,
        Job::Add(a, b) => get_value(jobs, a) + get_value(jobs, b),
        Job::Sub(a, b) => get_value(jobs, a) - get_value(jobs, b),
        Job::Mul(a, b) => get_value(jobs, a) * get_value(jobs, b),
        Job::Div(a, b) => get_value(jobs, a) / get_value(jobs, b),
    }
}
