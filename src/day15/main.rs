use std::collections::HashSet;

use advent_2022::utils::read_lines;

use regex::Regex;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let re =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();

    if let Ok(lines) = read_lines("./inputs/day15.txt") {
        let target_y = 2000000;
        let mut target_row_empty = HashSet::new();
        let mut target_row_occupied = HashSet::new();

        for line in lines {
            if let Ok(line) = line {
                if let Some(captures) = re.captures(&line) {
                    let sx = captures[1].parse::<i32>().unwrap();
                    let sy = captures[2].parse::<i32>().unwrap();
                    let bx = captures[3].parse::<i32>().unwrap();
                    let by = captures[4].parse::<i32>().unwrap();

                    if sy == target_y {
                        target_row_occupied.insert(sx);
                    }

                    if by == target_y {
                        target_row_occupied.insert(bx);
                    }

                    let dist = (sx - bx).abs() + (sy - by).abs();

                    let tdist = (sy - target_y).abs();

                    if dist >= tdist {
                        for i in sx - (dist - tdist)..sx + (dist - tdist) + 1 {
                            target_row_empty.insert(i);
                        }
                    }
                } else {
                    panic!("Line did not match the pattern");
                }
            }
        }

        println!(
            "There are {} cells where beacon cannot be present",
            target_row_empty.difference(&target_row_occupied).count()
        );
    }
}

fn part_two() {
    let re =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();

    if let Ok(lines) = read_lines("./inputs/day15.txt") {
        let mut coords = Vec::new();

        for line in lines {
            if let Ok(line) = line {
                if let Some(captures) = re.captures(&line) {
                    let sx = captures[1].parse::<i32>().unwrap();
                    let sy = captures[2].parse::<i32>().unwrap();
                    let bx = captures[3].parse::<i32>().unwrap();
                    let by = captures[4].parse::<i32>().unwrap();

                    let dist = (sx - bx).abs() + (sy - by).abs();

                    coords.push((sx, sy, dist));
                } else {
                    panic!("Line did not match the pattern");
                }
            }
        }

        for (idx, (x, y, d)) in coords.iter().enumerate() {
            for i in 0..*d + 1 {
                if !covered_by_sensor(&coords, idx, *x + i, *y + (d + 1) - i)
                    || !covered_by_sensor(&coords, idx, *x + d + 1 - i, *y - i)
                    || !covered_by_sensor(&coords, idx, *x - i, *y - (d + 1) + i)
                    || !covered_by_sensor(&coords, idx, *x - (d + 1) + i, *y + i)
                {
                    return;
                }
            }
        }

        println!("Coords are {:?}", coords);
    }
}

fn covered_by_sensor(coords: &[(i32, i32, i32)], skip: usize, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 || x > 4000000 || y > 4000000 {
        return true;
    }
    for (idx, (cx, cy, d)) in coords.iter().enumerate() {
        if idx != skip {
            let cd = (x - cx).abs() + (y - cy).abs();
            if cd <= *d {
                return true;
            }
        }
    }

    println!(
        "Found coord {}x{} with distance {}",
        x,
        y,
        x as u64 * 4000000 + y as u64
    );
    return false;
}
