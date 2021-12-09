use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// index is lifetime
type School = Vec<usize>;

fn main() {
    let mut school = vec![0; 9];

    let file = File::open("input.txt").expect("Could not open file");
    let lines = BufReader::new(file).lines();
    lines
        .map(|line| {
            line.expect("Could not read line")
                .split(',')
                .map(|elem| elem.parse::<usize>().expect("Could not parse number"))
                .collect::<Vec<usize>>()
        })
        .flatten()
        .for_each(|x| {
            school[x] += 1;
        });
    println!("Initial state: {:?}", school);

    // simulate days
    for day in 1..257 {
        let mut new_school = vec![0; 9];
        // lifetime = 0;
        new_school[6] += school[0];
        new_school[8] += school[0];

        for i in 1..school.len() {
            new_school[i - 1] += school[i];
        }
        school = new_school;
        println!("After {:2} days: {:?}", day, school);
        //println!("After {:2} days",day);
    }
    // calculate final number of fish
    let count: usize = school.iter().sum();
    println!("Complete count of fish is: {}", count);
}
