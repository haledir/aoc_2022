

fn main_one(){
    let max = include_str!("../../../inputs/day1.input")
        .split("\n\n")
        .map(|x| {
            return x.split("\n").flat_map(str::parse::<usize>).sum::<usize>();
        })
        .max();

    println!("max: {:?}", max);
}

fn main_two(){
    let mut max = include_str!("../../../inputs/day1.input")
        .split("\n\n")
        .map(|x| {
            return x.split("\n").flat_map(str::parse::<usize>).sum::<usize>();
        })
        .collect::<Vec<usize>>();

    max.sort_by(|a, b| b.cmp(a));

    println!("max: {:?}", max.iter().take(3).sum::<usize>());

}

fn main() {
    let input = include_str!("../../../inputs/day1.input");
    let lines = input.split("\n\n");

    let mut lines_parsed: Vec<u32> = lines
        .map(|line| line.split("\n")
            .flat_map(|num| num.parse::<u32>())
            .sum())
        .collect();
    
    lines_parsed.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", lines_parsed[0]);

    println!("Part 2: {:?}", lines_parsed.iter().take(3).sum::<u32>())
}
