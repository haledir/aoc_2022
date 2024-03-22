fn main () {
    part1();
    part2();
}

fn part1 () {
    let input = std::fs::read_to_string("../inputs/day6.example").unwrap();
    let mut idx = 0;
    let mut found = false;

    while !found {
        let mut result = 0;
        let mut id = 1;
        let current_slice = &input[idx..idx+4];
        for c in current_slice.chars() {
            if current_slice[id..].contains(c) {
                result = 0;
                break;
            }
            id += 1;
            result = idx + 4;
        }
        idx += 1;
        if result != 0 {
            println!("part 1: {}", result);
            break;
        }
    }
}

fn part2() {
    let input = std::fs::read_to_string("../inputs/day6.input").unwrap();
    let mut idx = 0;
    let mut found = false;

    while !found {
        let mut result = 0;
        let mut id = 1;
        let current_slice = &input[idx..idx+14];
        for c in current_slice.chars() {
            if current_slice[id..].contains(c) {
                result = 0;
                break;
            }
            id += 1;
            result = idx + 14;
        }
        idx += 1;
        if result != 0 {
            println!("part 2: {}", result);
            break;
        }
    }
}