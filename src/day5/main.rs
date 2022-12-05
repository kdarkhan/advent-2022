use std::collections::LinkedList;

use advent_2022::utils::read_lines;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day5.txt") {
        let mut finished_stacks = false;
        let mut array: [LinkedList<char>; 9] = Default::default();
        for line in lines {
            if let Ok(line) = line {
                if !finished_stacks {
                    if line.starts_with(" 1 ") {
                        finished_stacks = true;
                    } else {
                        for i in 0..9 {
                            if i * 4 <= line.len() {
                                let column = &line[i * 4..i * 4 + 3];
                                if column != "   " {
                                    array[i].push_back(column.chars().nth(1).unwrap());
                                }
                            }
                        }
                    }
                } else if line != "" {
                    let split: Vec<&str> = line.split(" ").collect();
                    let cnt = split[1].parse::<i32>().unwrap();
                    let from = split[3].parse::<usize>().unwrap();
                    let to = split[5].parse::<usize>().unwrap();

                    for _ in 0..cnt {
                        let char = array[from - 1].pop_front().unwrap();
                        array[to - 1].push_front(char);
                    }
                }
            }
        }

        for item in array.iter() {
            if !item.is_empty() {
                print!("{}", item.front().unwrap());
            }
        }
        println!("\nAll done");
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day5.txt") {
        let mut finished_stacks = false;
        let mut array: [LinkedList<char>; 9] = Default::default();
        for line in lines {
            if let Ok(line) = line {
                if !finished_stacks {
                    if line.starts_with(" 1 ") {
                        finished_stacks = true;
                    } else {
                        for i in 0..9 {
                            if i * 4 <= line.len() {
                                let column = &line[i * 4..i * 4 + 3];
                                if column != "   " {
                                    array[i].push_back(column.chars().nth(1).unwrap());
                                }
                            }
                        }
                    }
                } else if line != "" {
                    let split: Vec<&str> = line.split(" ").collect();
                    let cnt = split[1].parse::<i32>().unwrap();
                    let from = split[3].parse::<usize>().unwrap();
                    let to = split[5].parse::<usize>().unwrap();

                    let mut buffer: LinkedList<char> = Default::default();
                    for _ in 0..cnt {
                        buffer.push_back(array[from - 1].pop_front().unwrap())
                    }

                    for _ in 0..cnt {
                        array[to - 1].push_front(buffer.pop_back().unwrap());
                    }
                }
            }
        }

        for item in array.iter() {
            if !item.is_empty() {
                print!("{}", item.front().unwrap());
            }
        }
        println!("\nAll done");
    }
}
