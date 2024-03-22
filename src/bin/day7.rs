use std::thread::current;

fn part1() {
    let input = std::fs::read_to_string("../inputs/day7.input").unwrap();

    let mut stack = vec![
        ("/", 0)
    ];
    let report_amount = 100_000;
    let mut total_amount = 0;
    for line in input.lines().filter(|l| !l.is_empty()) {
        if line.starts_with("$ cd /") || line.starts_with("$ ls") {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir = &line[5..];
            if dir == ".." {
                let (_, amount) = stack.pop().unwrap();
                if amount <= report_amount {
                    total_amount += amount;
                }
                stack.last_mut().unwrap().1 += amount;
            } else {
                stack.push((dir, 0));
            }
            continue;
        }

        let (amount, _) = line.split_once(" ").unwrap();
        if let Ok(amount) = amount.parse::<usize>() {
            stack.last_mut().unwrap().1 += amount;
        }
    }
    println!("part 1: {}", total_amount);
}

fn part2() {
    let input = std::fs::read_to_string("../inputs/day7.input").unwrap();

    let mut stack = vec![
        ("/", 0)
    ];
    let mut final_countdown = vec![];
    let file_sys_total = 70000000;
    let space_to_delete = 30000000;
    let report_amount = 100_000;
    let mut total_amount = 0;
    for line in input.lines().filter(|l| !l.is_empty()) {
        if line.starts_with("$ cd /") || line.starts_with("$ ls") {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir = &line[5..];
            if dir == ".." {
                let (name, amount) = stack.pop().unwrap();
                if amount <= report_amount {
                    total_amount += amount;
                }
                stack.last_mut().unwrap().1 += amount;
                final_countdown.push((name, amount));
            } else {
                stack.push((dir, 0));
            }
            continue;
        }

        let (amount, _) = line.split_once(" ").unwrap();
        if let Ok(amount) = amount.parse::<usize>() {
            stack.last_mut().unwrap().1 += amount;
        }
    }

    while stack.len() > 0 {
        let (name, amount) = stack.pop().unwrap();
        final_countdown.push((name, amount));
        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += amount;
        }
    }
    let free_space = file_sys_total - final_countdown.last().unwrap().1;
    let required = space_to_delete - free_space;

    let total = final_countdown
        .into_iter()
        .filter(move |(_, amount)| *amount >= required)
        .map(|(_, amount)| {
            return amount;
        })
        .min();
    println!("part 2: {:?}", total);
}


fn main () {
    part1();
    part2();
}

