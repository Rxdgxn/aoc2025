use std::collections::HashMap;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Position(i64, i64, i64);

impl Position {
    fn diff(&self, other: &Self) -> i64 {
        (self.0 - other.0) * (self.0 - other.0) + (self.1 - other.1) * (self.1 - other.1) + (self.2 - other.2) * (self.2 - other.2)
    }
}

fn dfs(b: Position, new_id: usize, position_to_id: &mut HashMap<Position, usize>, neighbors: &HashMap<Position, Vec<Position>>) {
    let ns = neighbors.get(&b).unwrap();
    position_to_id.insert(b, new_id);

    for neigh in ns {
        if position_to_id[&neigh] != new_id {
            dfs(*neigh, new_id, position_to_id, neighbors);
        }
    }
}

pub fn part1(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();
    let boxes: Vec<Position> = content.lines().map(|l| {
        let v: Vec<i64> = l.split(',').map(|p| p.parse::<i64>().unwrap()).collect();
        Position(v[0], v[1], v[2])
    }).collect();

    let n = boxes.len();

    let mut diffs: Vec<(i64, Position, Position)> = Vec::new();

    for i in 0 .. n - 1 {
        for j in i + 1 .. n {
            diffs.push((boxes[i].diff(&boxes[j]), boxes[i], boxes[j]));
        }
    }

    diffs.sort_by_key(|t| t.0);

    let mut last_id = 0usize;
    let mut position_to_id: HashMap<Position, usize> = HashMap::new();
    let mut neighbors: HashMap<Position, Vec<Position>> = HashMap::new();
    let mut positions_per_id: HashMap<usize, i32> = HashMap::new();

    for i in 0 .. 1000 {
        neighbors.entry(diffs[i].1).or_insert(Vec::new()).push(diffs[i].2);
        neighbors.entry(diffs[i].2).or_insert(Vec::new()).push(diffs[i].1);

        if !position_to_id.contains_key(&diffs[i].1) && !position_to_id.contains_key(&diffs[i].2) {
            position_to_id.insert(diffs[i].1, last_id);
            position_to_id.insert(diffs[i].2, last_id);

            *positions_per_id.entry(last_id).or_insert(0) += 2;

            last_id += 1;
        }
        else if position_to_id.contains_key(&diffs[i].1) && !position_to_id.contains_key(&diffs[i].2) {
            let id = position_to_id[&diffs[i].1];

            position_to_id.insert(diffs[i].2, id);
            *positions_per_id.get_mut(&id).unwrap() += 1;
        }
        else if !position_to_id.contains_key(&diffs[i].1) && position_to_id.contains_key(&diffs[i].2) {
            let id = position_to_id[&diffs[i].2];

            position_to_id.insert(diffs[i].1, id);
            *positions_per_id.get_mut(&id).unwrap() += 1;
        }
        else if position_to_id[&diffs[i].1] != position_to_id[&diffs[i].2] {
            let new_id = position_to_id[&diffs[i].1];
            let old_count = positions_per_id[&position_to_id[&diffs[i].2]];

            positions_per_id.remove(&position_to_id[&diffs[i].2]);
            *positions_per_id.get_mut(&new_id).unwrap() += old_count;
            dfs(diffs[i].2, new_id, &mut position_to_id, &mut neighbors);
        }
    }

    let mut all_ids: Vec<usize> = positions_per_id.keys().cloned().collect();
    all_ids.sort_by_key(|id| -positions_per_id[id]);

    println!("Day 8, part 1: {}", positions_per_id[&all_ids[0]] * positions_per_id[&all_ids[1]] * positions_per_id[&all_ids[2]]);
}

pub fn part2(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();
    let boxes: Vec<Position> = content.lines().map(|l| {
        let v: Vec<i64> = l.split(',').map(|p| p.parse::<i64>().unwrap()).collect();
        Position(v[0], v[1], v[2])
    }).collect();

    let n = boxes.len();

    let mut diffs: Vec<(i64, Position, Position)> = Vec::new();

    let mut position_to_id: HashMap<Position, usize> = HashMap::new();
    let mut neighbors: HashMap<Position, Vec<Position>> = HashMap::new();
    let mut positions_per_id: HashMap<usize, i32> = HashMap::new();

    for i in 0 .. n - 1 {
        for j in i + 1 .. n {
            diffs.push((boxes[i].diff(&boxes[j]), boxes[i], boxes[j]));
        }
    }

    diffs.sort_by_key(|t| t.0);

    for id in 0 .. n {
        position_to_id.insert(boxes[id],id);
        positions_per_id.insert(id, 1);
    }

    let mut i = 0;
    loop {
        neighbors.entry(diffs[i].1).or_insert(Vec::new()).push(diffs[i].2);
        neighbors.entry(diffs[i].2).or_insert(Vec::new()).push(diffs[i].1);

        if position_to_id[&diffs[i].1] != position_to_id[&diffs[i].2] {
            let new_id = position_to_id[&diffs[i].1];
            let old_count = positions_per_id[&position_to_id[&diffs[i].2]];

            positions_per_id.remove(&position_to_id[&diffs[i].2]);
            *positions_per_id.get_mut(&new_id).unwrap() += old_count;
            dfs(diffs[i].2, new_id, &mut position_to_id, &neighbors);

            if positions_per_id.len() == 1 {
                println!("Day 8, part 2: {}", diffs[i].1.0 * diffs[i].2.0);
                break;
            }
        }

        i += 1;
    }
}