use std::{ops::Index, collections::HashMap};

fn main(){
    theo();
}

fn part2() {
    // X = lose
    // Y = draw
    // Z == win
    let mut result = 0;

    let input = include_str!("../../../inputs/day2.input");

    let lines = input.split("\n");

    for line in lines{
        let moves: Vec<&str> = line.split(" ").collect();
        let enemy: &str = moves.index(0);
        let player: &str = moves.index(1);
        match player {
            "X" => match enemy {
                "A" => result += 3,
                "B" => result += 1,
                "C" => result += 2,
                &_ => todo!()
            },
            "Y" => match enemy {
                "A" => result += 4,
                "B" => result += 5,
                "C" => result += 6,
                &_ => todo!()
            },
            "Z" => match enemy {
                "A" => result += 8,
                "B" => result += 9,
                "C" => result += 7,
                &_ => todo!()
            },
            &_ => println!("help")
        }

    }
    println!("result: {:?}",result);
}

fn part1() {

    let mut result = 0;

    let input = include_str!("../../../inputs/day2.input");

    let lines = input.split("\n");

    for line in lines{
        let moves: Vec<&str> = line.split(" ").collect();
        let enemy: &str = moves.index(0);
        let player: &str = moves.index(1);
        match player {
            "X" => match enemy {
                "A" => result += 4,
                "B" => result += 1,
                "C" => result += 7,
                &_ => todo!()
            },
            "Y" => match enemy {
                "A" => result += 8,
                "B" => result += 5,
                "C" => result += 2,
                &_ => todo!()
            },
            "Z" => match enemy {
                "A" => result += 3,
                "B" => result += 9,
                "C" => result += 6,
                &_ => todo!()
            },
            &_ => println!("help")
        }

    }
    println!("result: {:?}",result);
}

fn prime(){

}

fn theo(){

    let input = include_str!("../../../inputs/day2.input");
    let lines: Vec<&str> = input.split("\n").collect();

    let hm = getHashMap();
    let hm2 = getHashMap2();

    let mut result = 0;
    for line in &lines {
        if let Some(value) = hm.get(line){
            result += value;
        }
    }

    let mut result2 = 0;
    for line in lines {
        if let Some(value) = hm2.get(line){
            result2 += value;
        }
    }
    println!("Part 1: {}", result);
    println!("Part 2: {}", result2);

}

fn getHashMap() -> HashMap<&'static str, i32>{
    let mut map = HashMap::new();
    
    map.insert("A X", 1 + 3);
    map.insert("A Y", 2 + 6);
    map.insert("A Z", 3 + 0);
    map.insert("B X", 1 + 0);
    map.insert("B Y", 2 + 3);
    map.insert("B Z", 3 + 6);
    map.insert("C X", 1 + 6);
    map.insert("C Y", 2 + 0);
    map.insert("C Z", 3 + 3);

    return map
}

fn getHashMap2() -> HashMap<&'static str, i32>{
    let mut map = HashMap::new();
    
    map.insert("A X", 3 + 0);
    map.insert("A Y", 1 + 3);
    map.insert("A Z", 2 + 6);
    map.insert("B X", 1 + 0);
    map.insert("B Y", 2 + 3);
    map.insert("B Z", 3 + 6);
    map.insert("C X", 2 + 0);
    map.insert("C Y", 3 + 3);
    map.insert("C Z", 1 + 6);

    return map
}