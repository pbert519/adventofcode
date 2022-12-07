use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut input = BufReader::new(file);

    let mut stacks_str = String::new();
    while let Ok(bytes) = input.read_line(&mut stacks_str) {
        if bytes == 1 {
            break;
        }
    }

    let number_of_stacks = (stacks_str.lines().next().unwrap().len() + 1) / 4;
    println!("Number of Stacks: \n{}", number_of_stacks);

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for i in 0..number_of_stacks {
        let char_index = 1 + 4 * i;
        let mut stack = VecDeque::new();

        for line in stacks_str.lines() {
            if let Some(char) = line.chars().nth(char_index) {
                if char.is_uppercase() {
                    stack.push_front(char);
                }
            }
        }
        stacks.push(stack);
    }

    println!("Stacks: \n{:#?}", stacks);

    input.lines().for_each(|line| {
        let line = line.unwrap();
        let mut line = line.split(' ');

        let count = line.nth(1).unwrap().parse::<usize>().unwrap();
        let source = line.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let dest = line.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        println!("move {} from {} to {}", count, source, dest);

        for _ in 0..count {
            let moved_crate = stacks[source].pop_back().unwrap();
            stacks[dest].push_back(moved_crate);
        }
    });

    println!("Stacks move: \n{:#?}", stacks);

    let mut result = String::new();
    for stack in stacks {
        let top_crate = *stack.back().unwrap();
        result.push(top_crate);
    }

    println!("Result is: {}", result);

    // part 2

    let file = File::open("input.txt").unwrap();
    let mut input = BufReader::new(file);

    let mut stacks_str = String::new();
    while let Ok(bytes) = input.read_line(&mut stacks_str) {
        if bytes == 1 {
            break;
        }
    }

    let number_of_stacks = (stacks_str.lines().next().unwrap().len() + 1) / 4;
    println!("Number of Stacks: \n{}", number_of_stacks);

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for i in 0..number_of_stacks {
        let char_index = 1 + 4 * i;
        let mut stack = VecDeque::new();

        for line in stacks_str.lines() {
            if let Some(char) = line.chars().nth(char_index) {
                if char.is_uppercase() {
                    stack.push_front(char);
                }
            }
        }
        stacks.push(stack);
    }

    println!("Stacks: \n{:#?}", stacks);

    input.lines().for_each(|line| {
        let line = line.unwrap();
        let mut line = line.split(' ');

        let count = line.nth(1).unwrap().parse::<usize>().unwrap();
        let source = line.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let dest = line.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        println!("move {} from {} to {}", count, source, dest);

        let mut intermediate_stack = VecDeque::new();
        for _ in 0..count {
            let moved_crate = stacks[source].pop_back().unwrap();
            intermediate_stack.push_back(moved_crate);
        }
        for _ in 0..count {
            let moved_crate = intermediate_stack.pop_back().unwrap();
            stacks[dest].push_back(moved_crate);
        }
    });

    println!("Stacks move: \n{:#?}", stacks);

    let mut result = String::new();
    for stack in stacks {
        let top_crate = *stack.back().unwrap();
        result.push(top_crate);
    }

    println!("Result is: {}", result);
}
