use std::{cmp::max, collections::BTreeMap};

fn convert_range(range: &str) -> (u64, u64) {
    let (a, b) = range.split_once("-").unwrap();
    (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
}

// Could've just merged the ranges from the get go
pub fn part1(input_file: &str) {
    let content  = std::fs::read_to_string(input_file).unwrap();
    let mut it = content.lines();

    let mut map: BTreeMap<u64, u64> = BTreeMap::new();

    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        }

        let (a, b) = convert_range(line);

        // Keep maximal ranges
        if !map.contains_key(&a) {
            map.insert(a, b);
        }
        else {
            map.insert(a, max(b, map[&a]));
        }
    }

    let mut fresh = 0;

    while let Some(line) = it.next() {
        let x = line.parse::<u64>().unwrap();

        for key in map.keys() {
            if *key > x {
                break;
            }

            if map[key] >= x {
                fresh += 1;
                break;
            }
        }
    }

    println!("Day 5, part 1: {}", fresh);
}

pub fn part2(input_file: &str) {
    let content  = std::fs::read_to_string(input_file).unwrap();
    let mut it = content.lines();

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        }

        ranges.push(convert_range(line));
    }

    ranges.sort_by(|x, y| {
        x.0.cmp(&y.0)
    });

    let mut fresh = 0;
    let mut r = ranges[0];

    for i in 1 .. ranges.len() {
        if ranges[i].0 <= r.1 {
            r.1 = max(r.1, ranges[i].1);
        }
        else {
            fresh += r.1 - r.0 + 1;
            r = ranges[i];
        }
    }

    fresh += r.1 - r.0 + 1;

    println!("Day 5, part 2: {}", fresh);
}