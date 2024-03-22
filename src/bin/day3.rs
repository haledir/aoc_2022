use std::collections::HashMap;

fn main(){
    part1();
    part2();
}

fn part1(){
    let input = include_str!("../../../inputs/day3.input");

    let hm = get_hash_map();
    let lines = input.split("\n");
    let mut doubles: Vec<String> = Vec::new();
    let mut result = 0;

    for line in lines{
        let (part1, part2) = line.split_at(line.len()/2);
        let mut double: String = String::from("");
        for c in part1.chars() {
            if part2.contains(c){
                double = c.to_string();
                break;
            }
        }
        doubles.push(double);
    }
    for entry in doubles{
        if let Some(value) = hm.get(&entry){
            result += value
        }
    }
    println!("part1: {}", result)
}

fn part2(){
    let input = include_str!("../../../inputs/day3.input");

    let hm = get_hash_map();
    let lines = input.split("\n");
    let mut group: Vec<String> = Vec::new();
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut doubles: Vec<String> = Vec::new();
    let mut result = 0;
    let mut index = 1;
    
    for line in lines{
        group.push(line.to_string());
        index += 1;
        if index == 4 {
            index = 1;
            groups.push(group);
            group = Vec::new();
        }
    }
    for g in groups{
        let mut double: String = String::from("");
        for c in g[0].chars(){
            if g[1].contains(c) && g[2].contains(c){
                double = c.to_string();
                break;
            }
        }
        doubles.push(double);
    }
    for entry in doubles{
        if let Some(value) = hm.get(&entry){
            result += value
        }
    }
    println!("part2: {}", result);
}

fn get_hash_map() -> HashMap<String, i32>{
    let mut map = HashMap::new();
    let mut index = 1;
    for c in 'a'..='z'{
        map.insert(c.to_string(), index);
        index += 1;
    }
    for c in 'A'..='Z'{
        map.insert(c.to_string(), index);
        index += 1;
    }

    return map;
}