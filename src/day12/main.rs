use std::collections::VecDeque;

use advent_2022::utils::read_lines;

fn main() {
    part_one();
    part_two();
}

fn valid_coord(grid: &Vec<Vec<char>>, tx: i32, ty: i32) -> bool {
    if tx < 0 || tx >= grid.len() as i32 {
        return false;
    }

    if ty < 0 || ty >= grid[0].len() as i32 {
        return false;
    }

    return true;
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day12.txt") {
        let mut grid: Vec<Vec<char>> = Default::default();
        let (mut sx, mut sy) = (0, 0);
        let (mut tx, mut ty) = (0, 0);

        for (i, line) in lines.enumerate() {
            if let Ok(line) = line {
                let row = Vec::from_iter(line.chars());
                grid.push(row);

                if let Some(idx) = line.find("S") {
                    sx = i;
                    sy = idx;
                    grid[i][idx] = 'a';
                }

                if let Some(idx) = line.find("E") {
                    tx = i;
                    ty = idx;
                    grid[i][idx] = 'z';
                }
            }
        }

        println!("Found sx={} sy={} tx={} ty={}", sx, sy, tx, ty);

        let mut visited: Vec<Vec<i32>> = vec![vec![-1; grid[0].len()]; grid.len()];
        visited[sx][sy] = 0;
        let mut queue: VecDeque<(usize, usize)> = Default::default();
        queue.push_back((sx, sy));
        let deltas: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
        while let Some((cx, cy)) = queue.pop_front() {
            for (dx, dy) in deltas.iter() {
                let (mx, my) = (cx as i32 + dx, cy as i32 + dy);
                if valid_coord(&grid, mx, my) {
                    let (mx, my) = (mx as usize, my as usize);
                    if visited[mx][my] == -1 && grid[mx][my] as i32 - grid[cx][cy] as i32 <= 1 {
                        visited[mx][my] = visited[cx][cy] + 1;
                        queue.push_back((mx, my));

                        if mx == tx && my == ty {
                            println!("Distance is {} at {}x{}", visited[mx][my], mx, my);
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day12.txt") {
        let mut grid: Vec<Vec<char>> = Default::default();
        let (mut sx, mut sy) = (0, 0);

        for (i, line) in lines.enumerate() {
            if let Ok(line) = line {
                let row = Vec::from_iter(line.chars());
                grid.push(row);

                if let Some(idx) = line.find("E") {
                    sx = i;
                    sy = idx;
                    grid[i][idx] = 'z';
                }

                if let Some(idx) = line.find("S") {
                    grid[i][idx] = 'a';
                }
            }
        }

        println!("Found sx={} sy={}", sx, sy);

        let mut visited: Vec<Vec<i32>> = vec![vec![-1; grid[0].len()]; grid.len()];
        visited[sx][sy] = 0;
        let mut queue: VecDeque<(usize, usize)> = Default::default();
        queue.push_back((sx, sy));
        let deltas: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
        let mut best = i32::MAX;
        while let Some((cx, cy)) = queue.pop_front() {
            for (dx, dy) in deltas.iter() {
                let (mx, my) = (cx as i32 + dx, cy as i32 + dy);
                if valid_coord(&grid, mx, my) {
                    let (mx, my) = (mx as usize, my as usize);
                    if visited[mx][my] == -1 && grid[mx][my] as i32 - grid[cx][cy] as i32 >= -1 {
                        visited[mx][my] = visited[cx][cy] + 1;
                        queue.push_back((mx, my));

                        if grid[mx][my] == 'a' {
                            best = std::cmp::min(best, visited[mx][my]);
                        }
                    }
                }
            }
        }
        println!("Shortest step a-E is {}", best);
    }
}
