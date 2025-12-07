pub fn part1(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();
    let lines: Vec<Vec<&str>> = content.lines().map(|l| {
        l.split_whitespace().collect()
    }).collect();

    // Note: splitul produce si elemente empty

    let nr_columns = lines[0].len();
    let mut columns: Vec<Vec<u64>> = Vec::new();

    for _ in 0 .. nr_columns {
        columns.push(Vec::new());
    }

    for i in 0 .. lines.len() - 1 {
        for j in 0 .. nr_columns {
            columns[j].push(lines[i][j].parse().unwrap());
        }
    }

    let mut sum = 0u64;

    for i in 0 .. nr_columns {
        let op = lines.last().unwrap()[i];

        sum += match op {
            "+" => columns[i].iter().sum::<u64>(),
            "*" => columns[i].iter().product::<u64>(),
            _ => panic!("Something went wrong while parsing")
        };
    }

    println!("Day 6, part 1: {}", sum);
}

enum Operation {
    None,
    Add,
    Mul
}

pub fn part2(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();

    let digits: Vec<Vec<u8>> = content.lines().map(|l| {
        l.as_bytes().to_vec()
    }).collect();
    let lines = digits.len();

    let mut numbers: Vec<u64> = Vec::new();
    let mut ready: Operation = Operation::None;

    let mut sum = 0u64;

    for col in (0 .. digits[0].len()).rev() {
        let mut num = 0u64;

        for row in 0 .. lines - 1 {
            if digits[row][col] != 32 {
                num = num * 10 + digits[row][col] as u64 - 48;
            }

            if digits[row + 1][col] == '+' as u8  {
                ready = Operation::Add;
                break;
            }
            else if digits[row + 1][col] == '*' as u8 {
                ready = Operation::Mul;
                break;
            }
        }

        if num != 0 {
            numbers.push(num);
        }

        match ready {
            Operation::Add => {
                sum += numbers.iter().sum::<u64>();
                numbers.clear();
            },
            Operation::Mul => {
                sum += numbers.iter().product::<u64>();
                numbers.clear();
            },
            Operation::None => {}
        };

        ready = Operation::None;
    }

    println!("Day 6, part 2: {}", sum);
}