pub fn part1(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();
    let mut grid: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

    let mut rolls = 0;

    let m = grid.len();
    let n = grid[0].len();

    for i in 0 .. m {
        for j in 0 .. n {
            if grid[i][j] != '@' {
                continue;
            }

            let mut neighbors = 0;

            for ni in i as i32 - 1 ..= i as i32 + 1 {
                for nj in j as i32 - 1 ..= j as i32 + 1 {
                    if ni == i as i32 && nj == j as i32 {
                        continue;
                    }

                    if ni >= 0 && nj >= 0 && ni < m as i32 && nj < n as i32 && grid[ni as usize][nj as usize] == '@' {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                rolls += 1;
            }
        }
    }

    println!("Day 4, part 1: {}", rolls);
}

pub fn part2(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();
    let mut grid: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

    let mut rolls = 0;

    let m = grid.len();
    let n = grid[0].len();

    // Note: very dummy way
    let mut changed = true;

    while changed {
        changed = false;

        for i in 0 .. m {
            for j in 0 .. n {
                if grid[i][j] != '@' {
                    continue;
                }

                let mut neighbors = 0;

                for ni in i as i32 - 1 ..= i as i32 + 1 {
                    for nj in j as i32 - 1 ..= j as i32 + 1 {
                        if ni == i as i32 && nj == j as i32 {
                            continue;
                        }

                        if ni >= 0 && nj >= 0 && ni < m as i32 && nj < n as i32 && grid[ni as usize][nj as usize] == '@' {
                            neighbors += 1;
                        }
                    }
                }

                if neighbors < 4 {
                    rolls += 1;
                    grid[i][j] = '.';
                    changed = true;
                }
            }
        }

    }

    println!("Day 4, part 2: {}", rolls);
}