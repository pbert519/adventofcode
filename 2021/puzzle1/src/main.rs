use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let measurements_strings = BufReader::new(file).lines();
    let measurements = measurements_strings
        .map(|x| x.unwrap().parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .expect("Error converting to int");

    let last = measurements.iter();
    let compare = measurements.iter().skip(1);

    let increases = compare
        .zip(last)
        .filter(|(compare, last)| compare > last)
        .count();
    println!("The measurement increases {} times", increases);

    // part 2
    let last = measurements.windows(3);
    let compare = measurements.windows(3).skip(1);

    let filtered_increases = compare
        .zip(last)
        .filter(|(compare, last)| compare.iter().sum::<i32>() > last.iter().sum::<i32>())
        .count();
    println!(
        "The filtered measurement increases {} times",
        filtered_increases
    );
}
