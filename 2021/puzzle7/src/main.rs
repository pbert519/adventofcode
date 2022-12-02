use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_cost(positions: &Vec<i32>, target: i32) -> i32 {
    positions
        .iter()
        .fold(0, |acc, pos| acc + (pos - target).abs())
}

fn calculate_cost_increasing(positions: &Vec<i32>, target: i32) -> i32 {
    positions.iter().fold(0, |acc, pos| {
        let diff = (pos - target).abs() + 1;
        acc + (0..diff).fold(0, |acc, n| n + acc)
    })
}

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let input = BufReader::new(file).lines();
    // extract drawn numbers
    let numbers = input
        .map(|line| {
            line.expect("Could not read first line")
                .split(',')
                .map(|x| x.parse::<i32>().expect("Could not parse"))
                .collect::<Vec<i32>>()
        })
        .flatten()
        .collect::<Vec<i32>>();
    println!("{:?}", numbers);

    let mut min_cost = calculate_cost_increasing(&numbers, 0);
    for target in 0.. {
        let cost = calculate_cost_increasing(&numbers, target);
        if cost <= min_cost {
            min_cost = cost
        } else {
            println!(
                "Found minimum at position {} with cost {}",
                target - 1,
                min_cost
            );
            break;
        }
        println!("Position {} has cost {}", target, cost);
    }
}
