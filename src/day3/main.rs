use advent_2022::utils::read_lines;
use std::collections::HashSet;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day3.txt") {
        let mut priority_sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut set = HashSet::new();
                for (i, c) in ip.chars().enumerate() {
                    if i < ip.len() / 2 {
                        set.insert(c);
                    } else {
                        if set.contains(&c) {
                            set.remove(&c);
                            let ascii_code = c as u32;
                            let priority = if ascii_code > 96 {
                                ascii_code - 96
                            } else {
                                ascii_code - 38
                            };
                            priority_sum += priority;
                        }
                    }
                }
            }
        }
        println!("Total sum is {}", priority_sum);
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day3.txt") {
        let mut priority_sum = 0;
        let mut set = HashSet::new();
        for (idx, line) in lines.enumerate() {
            if let Ok(ip) = line {
                if idx % 3 == 0 {
                    set = HashSet::from_iter(ip.chars());
                } else {
                    let next: HashSet<char> = HashSet::from_iter(ip.chars());
                    set.retain(|&k| next.contains(&k));

                    if idx % 3 == 2 {
                        for x in &set {
                            let ascii_code = x.to_owned() as u32;
                            let priority = if ascii_code > 96 {
                                ascii_code - 96
                            } else {
                                ascii_code - 38
                            };
                            priority_sum += priority;
                        }
                    }
                }
            }
        }
        println!("Total sum is {}", priority_sum);
    }
}
