use advent_2022::utils::read_lines;

const ROWS: usize = 6;
const COLS: usize = 40;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day10.txt") {
        let mut reg = 1;
        let mut cycle = 0;
        let mut result = 0;
        for line in lines {
            if let Ok(line) = line {
                let mut diff = 0;
                if line != "noop" {
                    let mut split = line.split(" ");
                    split.next();
                    diff = split.next().unwrap().parse::<i32>().unwrap();

                    cycle += 1;
                    if cycle % 40 == 20 {
                        result += cycle * reg;
                    }
                }

                cycle += 1;

                if cycle % 40 == 20 {
                    result += cycle * reg;
                }

                reg += diff;
            }
        }
        println!("Result is {}", result);
    }
}

fn update_screen(screen: &mut [[char; COLS]; ROWS], cycle: i32, reg: i32) {
    let row = cycle as usize / COLS;
    let col = cycle as usize % COLS;

    if (reg - col as i32).abs() < 2 {
        screen[row][col] = '#';
    }
}

fn print_screen(screen: &[[char; COLS]; ROWS]) {
    for i in 0..ROWS {
        for j in 0..COLS {
            print!("{}", screen[i][j]);
        }
        print!("\n");
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day10.txt") {
        let mut reg = 1;
        let mut cycle = 0;
        let mut screen = [['.'; COLS]; ROWS];
        for line in lines {
            if let Ok(line) = line {
                let mut diff = 0;
                if line != "noop" {
                    let mut split = line.split(" ");
                    split.next();
                    diff = split.next().unwrap().parse::<i32>().unwrap();

                    cycle += 1;
                    update_screen(&mut screen, cycle - 1, reg);
                }

                cycle += 1;

                update_screen(&mut screen, cycle - 1, reg);
                reg += diff;
            }
        }
        print_screen(&screen);
    }
}
