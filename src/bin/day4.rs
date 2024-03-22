fn main(){
    part1();
    part2();
}

fn part1(){
    let mut result = 0;
    let _input = include_str!("../../../inputs/day4.input");
    let lines = _input.split("\n");
    for line in lines {
        let mut complete = false;
        let mut elfs_ranges: Vec<Vec<i32>> = Vec::new();
        let elfs = line.split(",");
        for elf in elfs{
            let range: Vec<i32> = elf.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
            let mut ranges: Vec<i32> = Vec::new();
            for number in range[0]..=range[1]{
                ranges.push(number);
            }
            elfs_ranges.push(ranges);
        }
        for x in &elfs_ranges[0] {
            if elfs_ranges[1].contains(&x) {
                complete = true;
            } else {
                complete = false;
                break;
            }
        }
        if complete == true {
            result += 1;
            continue;
        }
        complete = false;
        for x in &elfs_ranges[1] {
            if elfs_ranges[0].contains(&x) {
                complete = true;
            } else {
                complete = false;
                break;
            }
        }
        if complete == true {
            result += 1;
        }
    }
    println!("part 1: {}", result);
}

fn part2(){
    let mut result = 0;
    let _input = include_str!("../../../inputs/day4.input");
    let lines = _input.split("\n");
    for line in lines {
        let mut overlap = false;
        let mut elfs_ranges: Vec<Vec<i32>> = Vec::new();
        let elfs = line.split(",");
        for elf in elfs{
            let range: Vec<i32> = elf.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
            let mut ranges: Vec<i32> = Vec::new();
            for number in range[0]..=range[1]{
                ranges.push(number);
            }
            elfs_ranges.push(ranges);
        }
        for x in &elfs_ranges[0] {
            if elfs_ranges[1].contains(&x) {
                overlap = true;
            }
        }
        for x in &elfs_ranges[1] {
            if elfs_ranges[0].contains(&x) {
                overlap = true;
            }
        }
        if overlap == true{
            result += 1;
        }
    }
    println!("part 2: {}", result);
}