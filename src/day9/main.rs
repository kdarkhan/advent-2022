use std::collections::HashSet;

use advent_2022::utils::read_lines;

fn main() {
    part_one();
    part_two();
}

fn update_location(
    rope: &mut Vec<(i32, i32)>,
    dx: i32,
    dy: i32,
    cnt: i32,
    visited: &mut HashSet<(i32, i32)>,
) {
    for _ in 0..cnt {
        let (sx, sy) = rope[0];
        rope[0] = (sx + dx, sy + dy);

        let (mut hx, mut hy) = (0, 0);
        let len = rope.len();

        for (i, (tx, ty)) in rope.iter_mut().enumerate() {
            if i > 0 {
                let (ndx, ndy) = (hx - *tx, hy - *ty);

                if ndx.abs() > 1 || ndy.abs() > 1 {
                    (*tx, *ty) = (*tx + ndx.signum(), *ty + ndy.signum());
                }
            }
            if i == len - 1 {
                visited.insert((*tx, *ty));
            }

            (hx, hy) = (*tx, *ty);
        }
    }
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day9.txt") {
        let mut rope = vec![(0, 0); 2];
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((0, 0));

        for line in lines {
            if let Ok(line) = line {
                let split: Vec<&str> = line.split(" ").collect();
                let cnt = split.get(1).unwrap().parse::<i32>().unwrap();
                match *split.get(0).unwrap() {
                    "U" => update_location(&mut rope, -1, 0, cnt, &mut visited),
                    "D" => update_location(&mut rope, 1, 0, cnt, &mut visited),
                    "L" => update_location(&mut rope, 0, -1, cnt, &mut visited),
                    "R" => update_location(&mut rope, 0, 1, cnt, &mut visited),
                    dir => panic!("Unexpected direction {}", dir),
                }
            }
        }

        println!("Visited count is {}", visited.len());
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day9.txt") {
        let mut rope = vec![(0, 0); 10];
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((0, 0));

        for line in lines {
            if let Ok(line) = line {
                let split: Vec<&str> = line.split(" ").collect();
                let cnt = split.get(1).unwrap().parse::<i32>().unwrap();
                match *split.get(0).unwrap() {
                    "U" => update_location(&mut rope, -1, 0, cnt, &mut visited),
                    "D" => update_location(&mut rope, 1, 0, cnt, &mut visited),
                    "L" => update_location(&mut rope, 0, -1, cnt, &mut visited),
                    "R" => update_location(&mut rope, 0, 1, cnt, &mut visited),
                    dir => panic!("Unexpected direction {}", dir),
                }
            }
        }

        println!("Visited count is {}", visited.len());
    }
}
