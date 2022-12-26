static INPUT: &str = include_str!("../../assets/day11.txt");

#[derive(Debug, Clone)]
enum Operation {
    Plus(u64),
    Multiply(u64),
    MultiplySelf,
    PlusSelf,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    next_true: usize,
    next_false: usize,
}

fn main() {
    let mut monkeys = Vec::new();
    let mut monkey_product = 1;

    let mut input = INPUT.lines();
    while let Some(line) = input.next() {
        let items = input.next().unwrap()[18..]
            .split(", ")
            .map(|i| i.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let operation = input.next().unwrap();
        let number = operation[25..].parse::<u64>();
        let operation = match (&operation[23..24], number) {
            ("+", Ok(n)) => Operation::Plus(n),
            ("*", Ok(n)) => Operation::Multiply(n),
            ("+", _) => Operation::PlusSelf,
            ("*", _) => Operation::MultiplySelf,
            _ => panic!("Invalid operation"),
        };

        let divisor = input.next().unwrap()[21..].parse::<u64>().unwrap();
        monkey_product *= divisor;

        let next_true = input.next().unwrap()[29..].parse::<usize>().unwrap();
        let next_false = input.next().unwrap()[30..].parse::<usize>().unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            divisor,
            next_true,
            next_false,
        });

        println!("{} {:?}", line, monkeys.last().unwrap());

        input.next(); // empty line
    }

    let mut inspect_counts = vec![0u64; monkeys.len()];

    // Part 1: 20 rounds
    // Part 2: 10000 rounds
    for _ in 1..=10000 {
        for idx in 0..monkeys.len() {
            let monkey = monkeys[idx].clone();

            for item in monkey.items {
                inspect_counts[idx] += 1;

                let mut worry = item % monkey_product;

                worry = match monkey.operation {
                    Operation::Plus(n) => worry + (n % monkey_product),
                    Operation::Multiply(n) => worry * (n % monkey_product),
                    Operation::MultiplySelf => worry * worry,
                    Operation::PlusSelf => worry + worry,
                };

                worry %= monkey_product;

                // Part 1: worry /= 3;
                // Part 2: worry is not divided by 3
                //worry /= 3;

                let next = if worry % monkey.divisor == 0 {
                    monkey.next_true
                } else {
                    monkey.next_false
                };

                monkeys[next].items.push(worry);
            }

            monkeys[idx].items.clear();
        }
    }

    inspect_counts.sort_unstable();
    inspect_counts.reverse();

    for (idx, count) in inspect_counts.iter().enumerate() {
        println!("Monkey {} inspected items {} times.", idx, count);
    }

    println!("Monkey business: {}", inspect_counts[0] * inspect_counts[1]);
}
