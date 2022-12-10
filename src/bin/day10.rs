static INPUT: &str = include_str!("../../assets/day10.txt");

fn main() {
    let instructions = INPUT.lines().map(|l| {
        let mut split = l.split(' ');
        let inst = split.next().unwrap();
        if inst == "addx" {
            let amount = split.next().unwrap().parse::<i32>().unwrap();
            (inst, Some(amount))
        } else {
            (inst, None)
        }
    }).collect::<Vec<_>>();

    let mut signal_sum = 0;

    let mut x = 1;

    let mut wait_cycles = 0;
    let mut current_instruction = 0;

    for cycle in 1.. {
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                //println!("Cycle: {}, x: {}, {}", cycle, x, cycle * x);
                signal_sum += cycle * x;
            }
            _ => {}
        }

        let crt_pos: i32 = cycle % 40;
        if (x + 1).abs_diff(crt_pos) <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        if crt_pos == 0 {
            println!();
        }

        if let Some((inst, amount)) = instructions.get(current_instruction) {
            if wait_cycles > 0 {
                wait_cycles -= 1;
                if wait_cycles == 0 {
                    x += amount.unwrap();
                    current_instruction += 1;
                }
            } else {
                match inst {
                    &"addx" => wait_cycles = 1,
                    &"noop" => current_instruction += 1,
                    _ => panic!("Invalid instruction"),
                }
            }
        } else {
            break;
        }
    }

    println!("Signal sum: {}", signal_sum);
}