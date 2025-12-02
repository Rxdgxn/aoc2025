fn normalize(pos: &mut i32) {
    if *pos < 0 {
        *pos += 100;
    }
    else if *pos >= 100 {
        *pos -= 100;
    }
}

pub fn part1(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();

    let mut pos = 50;
    let mut pass = 0;

    for line in content.lines() {
        let mut mult = 1;

        if line.chars().nth(0).unwrap() == 'L' {
            mult *= -1;
        }

        let rot = &line[1 .. ].parse::<i32>().unwrap();

        pos += mult * (rot % 100);

        normalize(&mut pos);

        if pos == 0 {
            pass += 1;
        }
    }

    println!("Answer for day 1, part 1: {}", pass);
}

pub fn part2(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();

    let mut pos = 50;
    let mut pass = 0;

    for line in content.lines() {
        let mut mult = 1;

        if line.chars().nth(0).unwrap() == 'L' {
            mult *= -1;
        }

        let rot = &line[1 .. ].parse::<i32>().unwrap();

        pass += rot / 100;

        let prev = pos;
        pos += mult * (rot % 100);
        
        if prev == 0 {
            normalize(&mut pos);
            continue;
        }

        if pos <= 0 || pos >= 100 {
            pass += 1;
        }

        normalize(&mut pos);
    }

    println!("Answer for day 1, part 2: {}", pass);
}