use std::collections::HashSet;

use advent_2022::utils::read_lines;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day6.txt") {
        for line in lines {
            if let Ok(ip) = line {
                for idx in 0..ip.len() - 4 {
                    let x: HashSet<char> = HashSet::from_iter(ip[idx..idx + 4].chars());
                    if x.len() == 4 {
                        println!("[part one] Found start index {}", idx + 4);
                        return;
                    }
                }
            }
        }
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day6.txt") {
        for line in lines {
            if let Ok(ip) = line {
                for idx in 0..ip.len() - 14 {
                    let x: HashSet<char> = HashSet::from_iter(ip[idx..idx + 14].chars());
                    if x.len() == 14 {
                        println!("[part two] Found start index {}", idx + 14);
                        return;
                    }
                }
            }
        }
    }
}
