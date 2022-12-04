use advent_2022::utils::read_lines;

use regex::Regex;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut cnt = 0;
    if let Ok(lines) = read_lines("./inputs/day4.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let cap = re.captures(&ip).unwrap();
                let x1 = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let x2 = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let y1 = cap.get(3).unwrap().as_str().parse::<i64>().unwrap();
                let y2 = cap.get(4).unwrap().as_str().parse::<i64>().unwrap();

                if (x1 >= y1 && x2 <= y2) || (y1 >= x1 && y2 <= x2) {
                    cnt += 1;
                }
            }
        }

        println!("Overlapping count is {}", cnt);
    }
}

fn part_two() {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut cnt = 0;
    if let Ok(lines) = read_lines("./inputs/day4.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let cap = re.captures(&ip).unwrap();
                let x1 = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let x2 = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let y1 = cap.get(3).unwrap().as_str().parse::<i64>().unwrap();
                let y2 = cap.get(4).unwrap().as_str().parse::<i64>().unwrap();

                if (x1 >= y1 && x1 <= y2)
                    || (y1 >= x1 && y1 <= x2)
                    || (x2 >= y1 && x2 <= y2)
                    || (y2 >= x1 && y2 <= x2)
                {
                    cnt += 1;
                }
            }
        }

        println!("Overlapping count is {}", cnt);
    }
}
