use std::collections::{HashMap, HashSet};

use advent_2022::utils::read_lines;

fn main() {
    part_one();
    part_two();
}

fn parent_dir(s: &str) -> String {
    let r = s[..s.rfind("/").unwrap()].to_string();
    if r == "" {
        String::from("/")
    } else {
        r
    }
}

fn update_parents(cur_dir: &str, map: &mut HashMap<String, u64>, cnt: u64) {
    if cur_dir != "/" {
        let mut parent = parent_dir(&cur_dir);
        loop {
            *map.entry(parent.clone()).or_default() += cnt;
            if &parent == "/" {
                break;
            }
            parent = parent_dir(&parent);
        }
    }
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day7.txt") {
        let mut listing_files = false;
        let mut cur_dir = String::from("/");
        let mut map: HashMap<String, u64> = Default::default();
        let mut lsed_dirs: HashSet<String> = Default::default();
        let mut cnt: u64 = 0;
        for line in lines {
            if let Ok(line) = line {
                if line.starts_with("$") && listing_files {
                    *map.entry(cur_dir.clone()).or_default() += cnt;
                    listing_files = false;

                    update_parents(&cur_dir, &mut map, cnt);

                    cnt = 0;
                }
                if line.starts_with("$ cd ") {
                    if &line[5..] == "/" {
                        cur_dir = "/".to_string();
                    } else if &line[5..] == ".." {
                        cur_dir = parent_dir(&cur_dir);
                    } else if &cur_dir == "/" {
                        cur_dir = String::from("/") + &line[5..];
                    } else {
                        cur_dir = cur_dir + "/" + &line[5..];
                    }
                } else if line == "$ ls" {
                    if lsed_dirs.contains(&cur_dir) {
                        panic!("Done")
                    } else {
                        listing_files = true;
                        lsed_dirs.insert(cur_dir.clone());
                    }
                } else if listing_files {
                    let split: Vec<_> = line.split(" ").collect();

                    match split.get(0).unwrap().parse::<u64>() {
                        Ok(value) => cnt += value,
                        Err(_e) => {
                            if split.get(0).unwrap() != &"dir" {
                                panic!("Unexpected line {}", line);
                            }
                        }
                    }
                } else {
                    panic!("Unexpected state");
                }
            }
        }

        *map.entry(cur_dir.clone()).or_default() += cnt;
        update_parents(&cur_dir, &mut map, cnt);

        let mut result = 0;
        for (_, value) in map.iter() {
            if value <= &100000 {
                result += value;
            }
        }
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day7.txt") {
        let mut listing_files = false;
        let mut cur_dir = String::from("/");
        let mut map: HashMap<String, u64> = Default::default();
        let mut lsed_dirs: HashSet<String> = Default::default();
        let mut cnt: u64 = 0;
        for line in lines {
            if let Ok(line) = line {
                if line.starts_with("$") && listing_files {
                    *map.entry(cur_dir.clone()).or_default() += cnt;
                    listing_files = false;

                    update_parents(&cur_dir, &mut map, cnt);

                    cnt = 0;
                }
                if line.starts_with("$ cd ") {
                    if &line[5..] == "/" {
                        cur_dir = "/".to_string();
                    } else if &line[5..] == ".." {
                        cur_dir = parent_dir(&cur_dir);
                    } else if &cur_dir == "/" {
                        cur_dir = String::from("/") + &line[5..];
                    } else {
                        cur_dir = cur_dir + "/" + &line[5..];
                    }
                } else if line == "$ ls" {
                    if lsed_dirs.contains(&cur_dir) {
                        panic!("Done")
                    } else {
                        listing_files = true;
                        lsed_dirs.insert(cur_dir.clone());
                    }
                } else if listing_files {
                    let split: Vec<_> = line.split(" ").collect();

                    match split.get(0).unwrap().parse::<u64>() {
                        Ok(value) => cnt += value,
                        Err(_e) => {
                            if split.get(0).unwrap() != &"dir" {
                                panic!("Unexpected line {}", line);
                            }
                        }
                    }
                } else {
                    panic!("Unexpected state");
                }
            }
        }

        *map.entry(cur_dir.clone()).or_default() += cnt;
        update_parents(&cur_dir, &mut map, cnt);

        let mut smallest = std::u64::MAX;
        let root_size = *map.get("/").unwrap();
        for (_, value) in map.iter() {
            if 70000000 - root_size + *value > 30000000 && *value < smallest {
                smallest = *value;
            }
        }
        println!("Smallest dir size is {}", smallest);
    }
}
