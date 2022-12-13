use std::cmp::Ordering;
use std::collections::VecDeque;

use advent_2022::utils::read_lines;

fn main() {
    part_one();
    part_two();
}

#[derive(Debug, PartialEq)]
enum List {
    Num(i32),
    Items(Vec<List>),
}

fn in_order(left: &List, right: &List) -> Ordering {
    match (left, right) {
        (List::Num(l), List::Num(r)) => {
            if l == r {
                return Ordering::Equal;
            } else if l > r {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
        (List::Items(l), List::Items(r)) => {
            for i in 0..l.len() {
                if i >= r.len() {
                    return Ordering::Greater;
                }
                let acc = in_order(&l[i], &r[i]);
                if acc != Ordering::Equal {
                    return acc;
                }
            }
            return if l.len() < r.len() {
                Ordering::Less
            } else {
                Ordering::Equal
            };
        }
        (List::Items(_), List::Num(r)) => {
            return in_order(left, &List::Items(Vec::from([List::Num(*r)])))
        }
        (List::Num(l), List::Items(_)) => {
            return in_order(&List::Items(Vec::from([List::Num(*l)])), right)
        }
    }
}

fn parse_list(input: &str) -> List {
    let mut stack: VecDeque<List> = VecDeque::new();
    let mut list_acc = List::Items(Vec::new());

    let mut num_acc: String = String::new();

    for c in input.chars() {
        if c == '[' {
            stack.push_back(list_acc);
            list_acc = List::Items(Vec::new());
        } else if c == ']' {
            if let List::Items(ref mut to_add) = list_acc {
                if !num_acc.is_empty() {
                    to_add.push(List::Num(num_acc.parse().unwrap()));
                    num_acc.clear();
                }
                match stack.pop_back() {
                    Some(List::Items(mut items)) => {
                        items.push(list_acc);
                        list_acc = List::Items(items);
                    }
                    _ => panic!("Unexpected item of the top of the stack"),
                }
            }
        } else if c == ',' {
            if let List::Items(ref mut items) = list_acc {
                if !num_acc.is_empty() {
                    items.push(List::Num(num_acc.parse().unwrap()));
                    num_acc.clear();
                }
            } else {
                panic!("Unexpected item");
            }
        } else if c.is_digit(10) {
            num_acc.push(c);
        } else {
            panic!("unexpected char");
        }
    }
    return list_acc;
}

fn part_one() {
    if let Ok(lines) = read_lines("./inputs/day13.txt") {
        let mut left: String = String::new();
        let mut result = 0;
        for (i, line) in lines.enumerate() {
            if let Ok(line) = line {
                if i % 3 == 0 {
                    left = line;
                } else if i % 3 == 1 {
                    if in_order(&parse_list(&left), &parse_list(&line)) == Ordering::Less {
                        result += i / 3 + 1;
                    }
                }
            }
        }

        println!("In order indices sum is {}", result);
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./inputs/day13.txt") {
        let mut lists = Vec::new();
        for (i, line) in lines.enumerate() {
            if let Ok(line) = line {
                if i % 3 != 2 {
                    lists.push(parse_list(&line));
                }
            }
        }
        lists.push(parse_list("[[2]]"));
        lists.push(parse_list("[[6]]"));

        lists.sort_by(|a, b| in_order(a, b));

        let idx_2 = lists
            .iter()
            .position(|r| *r == parse_list("[[2]]"))
            .unwrap();
        let idx_6 = lists
            .iter()
            .position(|r| *r == parse_list("[[6]]"))
            .unwrap();

        println!("idx_2 is {}", idx_2 + 1);
        println!("idx_6 is {}", idx_6 + 1);
        println!("decoder key is {}", (idx_6 + 1) * (idx_2 + 1));
    }
}
