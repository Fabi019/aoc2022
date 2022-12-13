use std::{cmp::Ordering, collections::VecDeque, str::FromStr};

static INPUT: &str = include_str!("../../assets/day13.txt");

#[derive(Debug, Clone, PartialEq)]
enum Entry {
    Integer(u32),
    List(Vec<Entry>),
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: VecDeque<Vec<Entry>> = VecDeque::new();
        let mut result = Vec::new();

        let mut current_number = String::new();

        for ch in s.chars() {
            match ch {
                '[' => stack.push_back(Vec::new()),
                ']' | ',' => {
                    if !current_number.is_empty() {
                        let integer = current_number.parse::<u32>().unwrap();
                        if let Some(top) = stack.back_mut() {
                            top.push(Entry::Integer(integer));
                        } else {
                            result.push(Entry::Integer(integer));
                        }
                        current_number.clear();
                    }
                    if ch == ']' {
                        let nested_list = stack.pop_back().unwrap();
                        if let Some(top) = stack.back_mut() {
                            top.push(Entry::List(nested_list));
                        } else {
                            result.push(Entry::List(nested_list));
                        }
                    }
                }
                _ => current_number += &ch.to_string(),
            }
        }

        Ok(result[0].clone())
    }
}

impl Entry {
    fn compare_to(&self, right: &Self) -> Ordering {
        match (self, right) {
            (Entry::Integer(left), Entry::Integer(right)) => left.cmp(&right),
            (Entry::List(l), Entry::List(r)) => {
                for (left, right) in l.iter().zip(r.iter()) {
                    let order = left.compare_to(right);
                    if order != Ordering::Equal {
                        return order;
                    }
                }
                l.len().cmp(&r.len())
            }
            (Entry::Integer(_), Entry::List(r)) => {
                if r.is_empty() {
                    return Ordering::Greater;
                }
                Entry::List(vec![self.clone()]).compare_to(right)
            }
            (Entry::List(l), Entry::Integer(_)) => {
                if l.is_empty() {
                    return Ordering::Less;
                }
                self.compare_to(&Entry::List(vec![right.clone()]))
            }
        }
    }
}

fn main() {
    let mut packets = INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Entry>().unwrap())
        .collect::<Vec<_>>();

    let sum: usize = packets
        .chunks(2)
        .enumerate()
        .map(|(idx, pair)| (idx, pair[0].compare_to(&pair[1])))
        .filter(|o| o.1 == Ordering::Less)
        .map(|(idx, _)| idx + 1)
        .sum();

    println!("Part 1: {}", sum);

    let divider_1 = "[[2]]".parse::<Entry>().unwrap();
    let divider_2 = "[[6]]".parse::<Entry>().unwrap();

    packets.push(divider_1.clone());
    packets.push(divider_2.clone());

    packets.sort_by(|l, r| l.compare_to(r));

    let key: usize = packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| p == &divider_1 || p == &divider_2)
        .map(|(idx, _)| idx + 1)
        .product();

    println!("Part 2: {}", key);
}
