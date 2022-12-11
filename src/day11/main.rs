use advent_2022::utils::read_lines;

fn main() {
    part_one();
    part_two();
}

#[derive(Clone, Debug)]
enum Operation {
    Plus(u64),
    Mult(u64),
    Square,
}

impl Operation {
    fn apply(&self, operand: u64) -> u64 {
        match self {
            Operation::Plus(num) => operand + num,
            Operation::Mult(num) => operand * num,
            Operation::Square => operand * operand,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible_test: u64,
    if_true: usize,
    if_false: usize,
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day11.txt") {
        let mut monkeys: Vec<Monkey> = Vec::new();
        let mut items: Vec<u64> = Vec::new();
        let mut operation = Operation::Plus(0);
        let mut divisible_test = 0;
        let mut if_true = 0;
        for line in lines {
            if let Ok(line) = line {
                if line.starts_with("Monkey ") {
                    println!("Parsing monkey {}", &line[7..]);
                } else if line.starts_with("  Starting items:") {
                    items = Vec::from_iter(
                        line[18..]
                            .split(", ")
                            .map(|item| item.parse::<u64>().unwrap()),
                    );
                } else if line.starts_with("  Operation: new = old ") {
                    if line == "  Operation: new = old * old" {
                        operation = Operation::Square;
                    } else if line.chars().nth(23) == Some('*') {
                        operation = Operation::Mult(line[25..].parse::<u64>().unwrap());
                    } else if line.chars().nth(23) == Some('+') {
                        operation = Operation::Plus(line[25..].parse::<u64>().unwrap());
                    } else {
                        panic!("Unexpected operaiton");
                    }
                } else if line.starts_with("  Test: divisible by ") {
                    divisible_test = line[21..].parse().unwrap();
                } else if line.starts_with("    If true: throw to monkey ") {
                    if_true = line[29..].parse().unwrap();
                } else if line.starts_with("    If false: throw to monkey ") {
                    let if_false = line[30..].parse().unwrap();
                    monkeys.push(Monkey {
                        items: items.clone(),
                        operation: operation.clone(),
                        divisible_test,
                        if_true,
                        if_false,
                    })
                } else if !line.is_empty() {
                    panic!("Unexpected line");
                }
            }
        }

        let mut inspection_count: Vec<u32> = vec![0; monkeys.len()];
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let m = &monkeys[i].clone();
                monkeys[i].items = Vec::new();
                inspection_count[i] += m.items.len() as u32;
                for item in m.items.iter() {
                    let new_worry = m.operation.apply(*item) / 3;
                    if new_worry % m.divisible_test == 0 {
                        monkeys[m.if_true].items.push(new_worry);
                    } else {
                        monkeys[m.if_false].items.push(new_worry);
                    }
                }
            }
        }

        println!("Inspection count is {:?}", inspection_count);
        inspection_count.sort();
        let len = inspection_count.len();
        println!(
            "part one monkey business is {}x{}={}",
            inspection_count[len - 2],
            inspection_count[len - 1],
            inspection_count[len - 2] * inspection_count[len - 1]
        );
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day11.txt") {
        let mut monkeys: Vec<Monkey> = Vec::new();
        let mut items: Vec<u64> = Vec::new();
        let mut operation = Operation::Plus(0);
        let mut divisible_test = 0;
        let mut if_true = 0;
        let mut modulo = 1;
        for line in lines {
            if let Ok(line) = line {
                if line.starts_with("Monkey ") {
                    println!("Parsing monkey {}", &line[7..]);
                } else if line.starts_with("  Starting items:") {
                    items = Vec::from_iter(
                        line[18..]
                            .split(", ")
                            .map(|item| item.parse::<u64>().unwrap()),
                    );
                } else if line.starts_with("  Operation: new = old ") {
                    if line == "  Operation: new = old * old" {
                        operation = Operation::Square;
                    } else if line.chars().nth(23) == Some('*') {
                        operation = Operation::Mult(line[25..].parse::<u64>().unwrap());
                    } else if line.chars().nth(23) == Some('+') {
                        operation = Operation::Plus(line[25..].parse::<u64>().unwrap());
                    } else {
                        panic!("Unexpected operaiton");
                    }
                } else if line.starts_with("  Test: divisible by ") {
                    divisible_test = line[21..].parse().unwrap();
                    modulo *= divisible_test;
                } else if line.starts_with("    If true: throw to monkey ") {
                    if_true = line[29..].parse().unwrap();
                } else if line.starts_with("    If false: throw to monkey ") {
                    let if_false = line[30..].parse().unwrap();
                    monkeys.push(Monkey {
                        items: items.clone(),
                        operation: operation.clone(),
                        divisible_test,
                        if_true,
                        if_false,
                    })
                } else if !line.is_empty() {
                    panic!("Unexpected line");
                }
            }
        }

        let mut inspection_count: Vec<u32> = vec![0; monkeys.len()];

        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                let m = &monkeys[i].clone();
                monkeys[i].items = Vec::new();
                inspection_count[i] += m.items.len() as u32;
                for item in m.items.iter() {
                    let new_worry = m.operation.apply(*item) % modulo;
                    if new_worry % m.divisible_test == 0 {
                        monkeys[m.if_true].items.push(new_worry);
                    } else {
                        monkeys[m.if_false].items.push(new_worry);
                    }
                }
            }
        }

        inspection_count.sort();
        let len = inspection_count.len();
        println!(
            "part two monkey business is {}x{}={}",
            inspection_count[len - 2],
            inspection_count[len - 1],
            inspection_count[len - 2] as u128 * inspection_count[len - 1] as u128
        );
    }
}
