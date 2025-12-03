use std::collections::HashSet;

fn is_invalid1(x: u64) -> bool {
    let s = x.to_string();

    if s.len() % 2 != 0 {
        return false;
    }

    &s[0 .. s.len() / 2] == &s[s.len() / 2 .. ]
}

pub fn part1(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();
    let ranges = content.split(',');

    let mut sum = 0u64;

    // Brute force
    for range in ranges {
        let (a, b) = range.split_once('-').unwrap();
        let (a, b) = (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());

        for x in a ..= b {
            if is_invalid1(x) {
                sum += x;
            }
        }
    }

    println!("Day 2, part 1: {}", sum);
}

fn is_invalid2(x: u64) -> bool {
    let s = x.to_string();
    let len = s.len();

    for i in 1 ..= len / 2 {
        if len % i != 0 {
            continue;
        }

        let parts = len / i;

        let f = &s[0 .. i];
        let mut invalid = true;

        for j in 1 .. parts {
            if &s[j * i .. (j + 1) * i] != f {
                invalid = false;
                break;
            }
        }

        if invalid {
            return true;
        }
    }

    false
}

pub fn part2(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();
    let ranges = content.split(',');

    let mut sum = 0u64;

    // Brute force again
    for range in ranges {
        let (a, b) = range.split_once('-').unwrap();
        let (a, b) = (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());

        for x in a ..= b {
            if is_invalid2(x) {
                sum += x;
            }
        }
    }

    println!("Day 2, part 2: {}", sum);
}