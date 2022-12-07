use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
};

// A -> 0x41
// a -> 0x61

fn priority(c: char) -> i32 {
    if c.is_lowercase() {
        (c as i32) - 0x61 + 1
    } else {
        (c as i32) - 0x41 + 27
    }
}

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let input = BufReader::new(file).lines();

    let score: i32 = input
        .map(|line| {
            let line = line.unwrap();

            let middle = line.len() / 2;
            let comp_a = &line[0..middle];
            let comp_b = &line[middle..];

            let duplicate = comp_a
                .chars()
                .find(|c| comp_b.contains(*c))
                .expect("No duplicate char");
            priority(duplicate)
        })
        .sum();

    println!("The sum of priorities is {}", score);

    // ---------- part 2

    let file = File::open("input.txt").expect("Could not open file");
    let input = BufReader::new(file);

    let score = input
        .lines()
        .fuse()
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
        .chunks(3)
        .map(|group| {
            let a = &group[0].chars().collect::<BTreeSet<char>>();
            let b = &group[1].chars().collect::<BTreeSet<char>>();
            let c = &group[2].chars().collect::<BTreeSet<char>>();

            let letter = a
                .intersection(b)
                .copied()
                .collect::<BTreeSet<char>>()
                .intersection(c)
                .copied()
                .next()
                .unwrap();

            priority(letter)
        })
        .sum::<i32>();

    println!("The sum of priorities is {}", score);
}
