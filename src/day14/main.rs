use advent_2022::utils::read_lines;
use std::cmp::{max, min};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day14.txt") {
        let (sx, sy) = (500 as usize, 0 as usize);
        let (mut xmin, mut xmax, mut ymin, mut ymax) = (sx, sx, sy, sy);
        let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
        for line in lines {
            if let Ok(line) = line {
                paths.push(Vec::from_iter(line.split(" -> ").map(
                    |s| -> (usize, usize) {
                        let (left, right) = s.split_once(",").unwrap();
                        let left_i = left.parse::<usize>().unwrap();
                        let right_i = right.parse::<usize>().unwrap();

                        xmin = min(left_i, xmin);
                        xmax = max(left_i, xmax);
                        ymin = min(right_i, ymin);
                        ymax = max(right_i, ymax);
                        return (left_i, right_i);
                    },
                )));
            }
        }

        let (start_x, start_y) = (sx - xmin, sy - ymin);
        let (grid_width, grid_height) = ((xmax - xmin + 1) as usize, (ymax - ymin + 1) as usize);
        let mut grid = vec![vec!['.'; grid_height]; grid_width];

        for i in 0..paths.len() {
            let (mut cx, mut cy) = paths[i][0];
            (cx, cy) = (cx - xmin, cy - ymin);
            grid[cx][cy] = '#';
            for (tx, ty) in paths[i].iter().skip(1) {
                let (tx, ty) = (tx - xmin, ty - ymin);
                let (dx, dy) = (
                    (tx as i32 - cx as i32).signum(),
                    (ty as i32 - cy as i32).signum(),
                );
                if dx.abs() > 0 && dy.abs() > 0 {
                    panic!("Unexpected dx and dy");
                }

                while cx != tx || cy != ty {
                    (cx, cy) = ((cx as i32 + dx) as usize, (cy as i32 + dy) as usize);
                    grid[cx][cy] = '#';
                }
            }
        }

        let mut at_rest = 0;
        while fall_sand(&mut grid, grid_width, grid_height, start_x, start_y) {
            at_rest += 1;
        }
        println!("[part_one] at_rest count is {}", at_rest);
    }
}

fn fall_sand(grid: &mut Vec<Vec<char>>, width: usize, height: usize, cx: usize, cy: usize) -> bool {
    if cy >= height - 1 {
        return false;
    }
    if grid[cx][cy + 1] == '.' {
        return fall_sand(grid, width, height, cx, cy + 1);
    }
    if cx == 0 {
        return false;
    }
    if grid[cx - 1][cy + 1] == '.' {
        return fall_sand(grid, width, height, cx - 1, cy + 1);
    }
    if cx >= width - 1 {
        return false;
    }
    if grid[cx + 1][cy + 1] == '.' {
        return fall_sand(grid, width, height, cx + 1, cy + 1);
    }
    if grid[cx][cy] == '.' {
        grid[cx][cy] = 'o';
        return true;
    }

    return false;
}

fn fall_sand_part_two(
    grid: &mut Vec<Vec<char>>,
    cx: usize,
    cy: usize,
    tx: usize,
    ty: usize,
) -> bool {
    if grid[cx][cy + 1] == '.' {
        return fall_sand_part_two(grid, cx, cy + 1, tx, ty);
    }
    if grid[cx - 1][cy + 1] == '.' {
        return fall_sand_part_two(grid, cx - 1, cy + 1, tx, ty);
    }
    if grid[cx + 1][cy + 1] == '.' {
        return fall_sand_part_two(grid, cx + 1, cy + 1, tx, ty);
    }
    if grid[cx][cy] == '.' {
        grid[cx][cy] = 'o';
        return true;
    }
    return false;
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day14.txt") {
        let (sx, sy) = (500 as usize, 0 as usize);
        let (mut xmin, mut xmax, mut ymin, mut ymax) = (sx, sx, sy, sy);
        let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
        for line in lines {
            if let Ok(line) = line {
                paths.push(Vec::from_iter(line.split(" -> ").map(
                    |s| -> (usize, usize) {
                        let (left, right) = s.split_once(",").unwrap();
                        let left_i = left.parse::<usize>().unwrap();
                        let right_i = right.parse::<usize>().unwrap();

                        xmin = min(left_i, xmin);
                        xmax = max(left_i, xmax);
                        ymin = min(right_i, ymin);
                        ymax = max(right_i, ymax);
                        return (left_i, right_i);
                    },
                )));
            }
        }

        // hack, too lazy to compute the actual needed grid size
        xmin -= 200;
        xmax += 200;

        let (grid_width, grid_height) =
            ((xmax - xmin + 1) as usize, (ymax - ymin + 1) as usize + 2);
        let (start_x, start_y) = (sx - xmin, sy - ymin);

        println!("{}x{} in {}x{}", start_x, start_y, grid_width, grid_height);
        let mut grid = vec![vec!['.'; grid_height]; grid_width];

        for i in 0..grid_width {
            grid[i][grid_height - 1] = '#';
        }

        for i in 0..paths.len() {
            let (mut cx, mut cy) = paths[i][0];
            (cx, cy) = (cx - xmin, cy - ymin);
            grid[cx][cy] = '#';
            for (tx, ty) in paths[i].iter().skip(1) {
                let (tx, ty) = (tx - xmin, ty - ymin);
                let (dx, dy) = (
                    (tx as i32 - cx as i32).signum(),
                    (ty as i32 - cy as i32).signum(),
                );
                if dx.abs() > 0 && dy.abs() > 0 {
                    panic!("Unexpected dx and dy");
                }

                while cx != tx || cy != ty {
                    (cx, cy) = ((cx as i32 + dx) as usize, (cy as i32 + dy) as usize);
                    grid[cx][cy] = '#';
                }
            }
        }

        let mut at_rest = 0;
        while fall_sand_part_two(&mut grid, start_x, start_y, start_x, start_y) {
            at_rest += 1;
        }
        println!("[part_two] at_rest count is {}", at_rest);
    }
}
