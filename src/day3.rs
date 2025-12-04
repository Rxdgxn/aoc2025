pub fn part1(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();
    let mut joltage = 0;

    for line in content.lines() {
        let digits: Vec<char> = line.chars().collect();
        let mut max_digit = 0;
        let mut max_index = 0usize;

        for i in 0 .. digits.len() {
            let d = digits[i] as i32 - 48;
            
            if d > max_digit {
                max_digit = d;
                max_index = i;
            }
        }

        // Search for second biggest digit after the biggest
        let mut max_after = -1;

        for i in max_index + 1 .. digits.len() {
            let d = digits[i] as i32 - 48;
            if d > max_after {
                max_after = d;
            }
        }

        if max_after != -1 {
            joltage += max_digit * 10 + max_after;
            continue;
        }

        // Search for second biggest digit before the biggest
        let mut max_before = -1;

        for i in 0 .. max_index {
            let d = digits[i] as i32 - 48;
            if d > max_before {
                max_before = d;
            }
        }

        joltage += max_before * 10 + max_digit;
    }

    println!("Day 3, part 1: {}", joltage);
}

fn get_first_max_index(portion: &[i32]) -> usize {
    let mut ret = 0;
    let mut max = 0;

    for i in 0 .. portion.len() {
        if portion[i] > max {
            max = portion[i];
            ret = i;
        }
    }

    ret
}

fn add_bateries(digits: &[i32], l: usize, r: usize, rem: &mut usize, batteries: &mut Vec<usize>) {
    if *rem == 0 || l > r {
        return;
    }

    let i = l + get_first_max_index(&digits[l ..= r]);
    batteries.push(i);
    *rem -= 1;

    add_bateries(digits, i + 1, r, rem, batteries);
    
    if i > 0 {
        add_bateries(digits, l, i - 1, rem, batteries);
    }
}

pub fn part2(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();
    let mut joltage = 0u64;

    for line in content.lines() {
        let digits: Vec<i32> = line.chars().map(|c| c as i32 - 48).collect();
        let mut batteries: Vec<usize> = Vec::new();

        let mut rem = 12;

        add_bateries(&digits, 0, digits.len() - 1, &mut rem, &mut batteries);

        assert_eq!(batteries.len(), 12);

        batteries.sort();

        let mut p = 0u64;

        for b in batteries {
            p = p * 10 + digits[b] as u64;
        }

        joltage += p;
    }

    println!("Day 3, part 2: {}", joltage);
}