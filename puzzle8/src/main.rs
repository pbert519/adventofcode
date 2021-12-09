use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;

fn decode(input: &[String]) -> i32 {
    let (unqiue, digits): (BTreeSet<_>, BTreeSet<_>) = input
        .iter()
        .map(|str| str.chars().collect::<BTreeSet<_>>())
        .partition(|set| (set.len() < 5) || (set.len() > 6));

    // map already known digits
    let mut assigned = unqiue
        .into_iter()
        .map(|digit| match digit.len() {
            2 => (1, digit),
            3 => (7, digit),
            4 => (4, digit),
            7 => (8, digit),
            _ => panic!("Could not assign unique digit"),
        })
        .collect::<BTreeMap<u8, BTreeSet<char>>>();

    // extract 6, only digit with 6 segments and not a superset of 1
    let (assign, digits): (BTreeSet<_>, BTreeSet<_>) = digits
        .into_iter()
        .partition(|d| (d.len() == 6) && !d.is_superset(assigned.index(&1u8)));
    assigned.insert(6, assign.into_iter().last().unwrap());
    // extract 9, only digit with 6 segments and a superset of 4
    let (assign, digits): (BTreeSet<_>, BTreeSet<_>) = digits
        .into_iter()
        .partition(|d| (d.len() == 6) && d.is_superset(assigned.index(&4u8)));
    assigned.insert(9, assign.into_iter().last().unwrap());
    // extract 0, only remain digit with 6 segments
    let (assign, digits): (BTreeSet<_>, BTreeSet<_>) =
        digits.into_iter().partition(|d| d.len() == 6);
    assigned.insert(0, assign.into_iter().last().unwrap());

    // extract 3, only digit with 5 segments and a superset of 1
    let (assign, digits): (BTreeSet<_>, BTreeSet<_>) = digits
        .into_iter()
        .partition(|d| (d.len() == 5) && d.is_superset(assigned.index(&1u8)));
    assigned.insert(3, assign.into_iter().last().unwrap());
    // extract 5, only digit with 5 segments and two different segments as 4 (2 has three different segments)
    let (assign, digits): (BTreeSet<_>, BTreeSet<_>) = digits
        .into_iter()
        .partition(|d| (d.len() == 5) && ((d - assigned.index(&4u8)).len() == 2));
    assigned.insert(5, assign.into_iter().last().unwrap());
    // extract 2, the only remaining unassigned digit
    assigned.insert(2, digits.into_iter().last().unwrap());

    (10..14)
        .map(|i| {
            let digit = input[i].chars().collect::<BTreeSet<_>>();
            assigned
                .iter()
                .filter(|(_, value)| *value == &digit)
                .map(|(key, _)| key)
                .collect::<Vec<_>>()[0]
        })
        .fold(0, |acc, digit| acc * 10 + (*digit as i32))
}

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let lines = BufReader::new(file).lines();
    let re = Regex::new(r"([a-z]+) ([a-z]+) ([a-z]+) ([a-z]+) ([a-z]+) ([a-z]+) ([a-z]+) ([a-z]+) ([a-z]+) ([a-z]+) \| ([a-z]+) ([a-z]+) ([a-z]+) ([a-z]+)").expect("Regex expression not valid");

    let output_values: i32 = lines
        .map(|l| {
            re.captures_iter(&l.unwrap())
                .map(|cap| {
                    let input = cap
                        .iter()
                        .skip(1)
                        .map(|c| c.unwrap().as_str().to_string())
                        .collect::<Vec<String>>();
                    decode(&input)
                })
                .sum::<i32>()
        })
        .sum();
    println!("{:?}", output_values);
}
