use std::collections::HashSet;

use advent_2022::utils::read_lines;
use std::cmp::max;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day8.txt") {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in lines {
            if let Ok(line) = line {
                let row = Vec::from_iter(line.chars());
                grid.push(row);
            }
        }

        let mut set: HashSet<(usize, usize)> = HashSet::new();
        let mut cnt = 0;
        let rows = grid.len();
        let cols = grid.get(0).unwrap().len();
        // left to right
        for (i, v) in grid.iter().enumerate() {
            let mut high = ' ';
            for (j, item) in v.iter().enumerate() {
                if high < *item {
                    high = *item;

                    if !set.contains(&(i, j)) {
                        cnt += 1;

                        set.insert((i, j));
                    }
                }
            }
        }

        // right to left
        for (i, v) in grid.iter().enumerate() {
            let mut high = ' ';
            for (j, item) in v.iter().enumerate().rev() {
                if high < *item {
                    high = *item;

                    if !set.contains(&(i, j)) {
                        cnt += 1;

                        set.insert((i, j));
                    }
                }
            }
        }

        // top to bottom
        for j in 0..cols {
            let mut high = ' ';
            for i in 0..rows {
                let item = grid.get(i).unwrap().get(j).unwrap();
                if high < *item {
                    high = *item;

                    if !set.contains(&(i, j)) {
                        cnt += 1;

                        set.insert((i, j));
                    }
                }
            }
        }

        // bottom to top
        for j in 0..cols {
            let mut high = ' ';
            for i in (0..rows).rev() {
                let item = grid.get(i).unwrap().get(j).unwrap();
                if high < *item {
                    high = *item;

                    if !set.contains(&(i, j)) {
                        cnt += 1;

                        set.insert((i, j));
                    }
                }
            }
        }

        println!("Count is {}", cnt);
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day8.txt") {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in lines {
            if let Ok(line) = line {
                let row = Vec::from_iter(line.chars());
                grid.push(row);
            }
        }

        let rows = grid.len();
        let cols = grid.get(0).unwrap().len();
        let mut best = 0;

        // top to bottom
        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                let item = *grid.get(i).unwrap().get(j).unwrap();
                let mut left_count = 0;
                let mut right_count = 0;
                let mut up_count = 0;
                let mut down_count = 0;

                // left
                for k in (0..j).rev() {
                    let cur = *grid.get(i).unwrap().get(k).unwrap();
                    left_count += 1;
                    if cur >= item {
                        break;
                    }
                }
                // right
                for k in j + 1..cols {
                    let cur = *grid.get(i).unwrap().get(k).unwrap();
                    right_count += 1;
                    if cur >= item {
                        break;
                    }
                }
                // up
                for k in (0..i).rev() {
                    let cur = *grid.get(k).unwrap().get(j).unwrap();
                    up_count += 1;
                    if cur >= item {
                        break;
                    }
                }
                // down
                for k in i + 1..rows {
                    let cur = *grid.get(k).unwrap().get(j).unwrap();
                    down_count += 1;
                    if cur >= item {
                        break;
                    }
                }

                best = max(best, left_count * right_count * up_count * down_count);
            }
        }

        println!("Best scenic score is {}", best);
    }
}
