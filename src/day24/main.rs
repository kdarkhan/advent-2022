use advent_2022::utils::read_lines;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day3.txt") {
        for line in lines {
            if let Ok(ip) = line {
                println!("Line is {}", ip)
            }
        }
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day3.txt") {
        for line in lines {
            if let Ok(ip) = line {
                println!("Line is {}", ip)
            }
        }
    }
}
