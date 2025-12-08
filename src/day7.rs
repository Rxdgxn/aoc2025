use std::collections::{HashSet, HashMap};

fn dfs1(grid: &Vec<Vec<u8>>, (i, j): (usize, usize), m: usize, n: usize, hits: &mut i32, vis: &mut HashSet<(usize, usize)>) {
    if i == m - 1 || vis.contains(&(i, j)) {
        return;
    }

    vis.insert((i, j));

    if grid[i + 1][j] == '.' as u8 {
        dfs1(grid, (i + 1, j), m, n, hits, vis);
    }
    else {
        *hits += 1;

        if j as isize - 1 >= 0 {
            dfs1(grid, (i + 1, j - 1), m, n, hits, vis);
        }
        if j + 1 < n {
            dfs1(grid, (i + 1, j + 1), m, n, hits, vis);
        }
    }
}

fn get_start(grid: &Vec<Vec<u8>>, m: usize, n: usize) -> (usize, usize) {
    for i in 0 .. m {
        for j in 0 .. n {
            if grid[i][j] == 'S' as u8 {
                return (i, j);
            }
        }
    }

    unreachable!()
}

pub fn part1(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();
    let grid: Vec<Vec<u8>> = content.lines().map(|l| {
        l.as_bytes().to_vec()
    }).collect();

    let m = grid.len();
    let n = grid[0].len();

    let start_pos = get_start(&grid, m, n);
    let mut hits = 0;
    let mut set = HashSet::new();

    dfs1(&grid, start_pos, m, n, &mut hits, &mut set);

    println!("Day 7, part 1: {}", hits);
}

fn dfs2(grid: &Vec<Vec<u8>>, (i, j): (usize, usize), m: usize, n: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    if memo.contains_key(&(i, j)) {
        return memo[&(i, j)];
    }

    if i == m - 1 {
        memo.insert((i, j), 1);
        return 1;
    }

    let mut timelines = 0;

    if grid[i + 1][j] == '.' as u8 {
        timelines += dfs2(grid, (i + 1, j), m, n, memo);
    }
    else {
        if j as isize - 1 >= 0 {
            timelines += dfs2(grid, (i + 1, j - 1), m, n, memo);
        }
        if j + 1 < n {
            timelines += dfs2(grid, (i + 1, j + 1), m, n, memo);
        }
    }

    memo.insert((i, j), timelines);
    timelines
}

pub fn part2(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();
    let grid: Vec<Vec<u8>> = content.lines().map(|l| {
        l.as_bytes().to_vec()
    }).collect();

    let m = grid.len();
    let n = grid[0].len();

    let start_pos = get_start(&grid, m, n);
    let mut map = HashMap::new();

    let timelines = dfs2(&grid, start_pos, m, n, &mut map);

    println!("Day 7, part 2: {}", timelines);
}