use std::str::FromStr;

use anyhow::Result;

struct MoveOrders {
    move_orders: Vec<MoveOrder>,
}

impl FromStr for MoveOrders {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut orders: Vec<MoveOrder> = Vec::new();
        for line in s.lines() {
            orders.push(MoveOrder::from_str(line)?);
        }

        return Ok(MoveOrders {
            move_orders: orders
        });
    }
}
#[derive(Debug)]
struct MoveOrder {
    amount: usize,
    crane_from_idx: usize,
    crane_to_idx: usize,
}

impl MoveOrder {
    fn new(amount: usize, crane_from_idx: usize, crane_to_idx: usize) -> Self {
        return MoveOrder { 
            amount, 
            crane_from_idx, 
            crane_to_idx,
        }
    }
}

impl FromStr for MoveOrder {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.split(" ").collect();
        return Ok(MoveOrder::new(input[1].parse::<usize>()?, input[3].parse::<usize>()?, input[5].parse::<usize>()?) )
    }
}

fn main() {
    part1().expect("msg");
    part2().expect("msg");
}

fn part1() -> Result<()>{
    let input = std::fs::read_to_string("../inputs/day5.input")?;

    let (stacks, moves_input) = input.split_once("\n\n").expect("must be there");

    let mut crane: Vec<Vec<char>> = Vec::new();
    let moves: MoveOrders = MoveOrders::from_str(moves_input)?;
    let mut crane_idx: usize;
    let mut idx: usize;

    for line in stacks.lines().rev() {
        crane_idx = 0;
        idx = 0;
        while idx < line.len() {
            if line[idx..].starts_with("[") {
                let c = line.chars().nth(idx + 1).expect("pls be there");
                if crane.len() <= crane_idx{
                    crane.push(Vec::new());
                }
                crane[crane_idx].push(c)
            }
            crane_idx += 1;
            idx += 4;
        }
    }

    for order in moves.move_orders {
        for _ in 0..order.amount {
            let value = crane[order.crane_from_idx - 1].pop().expect("value is ther");
            crane[order.crane_to_idx - 1].push(value);
        }
    }

    print!("part1: ");
    for cra in crane {
        print!("{}", cra.last().unwrap());
    }
    println!("");

    return Ok(());
}

fn part2() -> Result<()>{
    let input = std::fs::read_to_string("../inputs/day5.input")?;

    let (stacks, moves_input) = input.split_once("\n\n").expect("must be there");

    let mut crane: Vec<Vec<char>> = Vec::new();
    let moves: MoveOrders = MoveOrders::from_str(moves_input)?;
    let mut crane_idx: usize;
    let mut idx: usize;

    for line in stacks.lines().rev() {
        crane_idx = 0;
        idx = 0;
        while idx < line.len() {
            if line[idx..].starts_with("[") {
                let c = line.chars().nth(idx + 1).expect("pls be there");
                if crane.len() <= crane_idx{
                    crane.push(Vec::new());
                }
                crane[crane_idx].push(c)
            }
            crane_idx += 1;
            idx += 4;
        }
    }

    for order in moves.move_orders {
        let new_len = crane[order.crane_from_idx - 1].len() - order.amount;
        let mut elements: Vec<char> = Vec::new();
        for removed_element in crane[order.crane_from_idx - 1].drain(new_len..) {
            elements.push(removed_element);
        }
        for ele in elements {
            crane[order.crane_to_idx - 1].push(ele);
        }
    
    }

    print!("part2: ");
    for cra in crane {
        print!("{}", cra.last().unwrap());
    }
    println!("");

    return Ok(());
}