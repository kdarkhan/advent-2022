use advent_2022::utils::read_lines;
use std::collections::HashMap;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day2.txt") {
        let mut score: i64 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<_> = ip.split(" ").collect();
                let left = split[0];
                let right = split[1];

                match right {
                    "X" => score += 1,
                    "Y" => score += 2,
                    "Z" => score += 3,
                    _ => panic!("Unexpected item"),
                }

                if (left == "A" && right == "X")
                    || (left == "B" && right == "Y")
                    || (left == "C" && right == "Z")
                {
                    score += 3;
                } else if (left == "A" && right == "Y")
                    || (left == "B" && right == "Z")
                    || (left == "C" && right == "X")
                {
                    score += 6;
                }
            }
        }

        println!("score is {}", score);
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day2.txt") {
        let mut score: i64 = 0;

        let score_mapping = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
        let win_mapping = HashMap::from([("A", "B"), ("B", "C"), ("C", "A")]);
        let lose_mapping = HashMap::from([("A", "C"), ("B", "A"), ("C", "B")]);

        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<_> = ip.split(" ").collect();
                let left = split[0];
                let right = split[1];

                if right == "Y" {
                    score += 3;
                } else if right == "Z" {
                    score += 6;
                }

                if right == "Y" {
                    score += score_mapping.get(left).unwrap();
                } else if right == "X" {
                    score += score_mapping.get(lose_mapping.get(left).unwrap()).unwrap();
                } else {
                    score += score_mapping.get(win_mapping.get(left).unwrap()).unwrap();
                }
            }
        }

        println!("score is {}", score);
    }
}
