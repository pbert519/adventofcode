use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_opening(c: char) -> bool {
    (c == '(') || (c == '[') || (c == '{') || (c == '<')
}
fn is_closing(c: char) -> bool {
    (c == ')') || (c == ']') || (c == '}') || (c == '>')
}
fn get_closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("invalid character"),
    }
}

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let input = BufReader::new(file).lines();

    let mut incomplete: Vec<Vec<char>> = Vec::new();
    let map = input
        .map(|line| {
            let mut result = Ok(());
            let mut stack: Vec<char> = Vec::with_capacity(30);
            for char in line.unwrap().chars() {
                if is_opening(char) {
                    stack.push(char);
                } else if is_closing(char) {
                    if get_closing(stack.pop().unwrap()) != char {
                        result = Err(char);
                        break;
                    }
                } else {
                    panic!("Invalid character");
                }
            }
            if result.is_ok() {
                incomplete.push(stack);
            }
            result
        })
        .collect::<Vec<_>>();

    let part1_score: i32 = map
        .iter()
        .filter(|x| x.is_err())
        .map(|x| match x {
            Err(')') => 3,
            Err(']') => 57,
            Err('}') => 1197,
            Err('>') => 25137,
            _ => panic!("invalid character"),
        })
        .sum();

    let mut incomplete = incomplete
        .into_iter()
        .map(|line| {
            line.into_iter()
                .rev()
                .map(|char| get_closing(char))
                .fold(0u64, |acc, char| {
                    acc * 5
                        + match char {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => panic!("invalid character"),
                        }
                })
        })
        .collect::<Vec<_>>();
    incomplete.sort_unstable();
    let index = incomplete.len() / 2;

    println!("{}", part1_score);
    println!("{:?}", incomplete[index]);
}
