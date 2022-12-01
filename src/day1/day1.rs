use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    part_one();
    part_two();

}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day1.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut best: i64 = 0;
        let mut cur_sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    if cur_sum > best {
                        best = cur_sum;
                    }
                    cur_sum = 0;
                } else {
                    cur_sum += ip.parse::<i64>().unwrap();
                }
            }
        }

        if cur_sum > best {
            best = cur_sum;
        }

        println!("best sum is {}", best);
    }
}


fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day1.txt") {
        let mut vec = Vec::new();

        let mut cur_sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    vec.push(cur_sum);
                    cur_sum = 0;
                } else {
                    cur_sum += ip.parse::<i64>().unwrap();
                }
            }
        }

        vec.push(cur_sum);

        vec.sort();
            println!("{}", vec[vec.len() - 1] + vec[vec.len() - 2] + vec[vec.len() - 3]);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
