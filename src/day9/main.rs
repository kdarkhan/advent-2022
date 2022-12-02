use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
