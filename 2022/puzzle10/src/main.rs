use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let input = BufReader::new(file);

    let mut x = 1;
    let mut x_history = vec![1; 1];

    for input in input.lines() {
        let input = input.unwrap();
        if input.contains("noop") {
            x_history.push(x);
        } else {
            x_history.push(x);
            let add: i32 = input.split(' ').nth(1).unwrap().parse().unwrap();
            x += add;
            x_history.push(x);
        }
    }

    let signal_strength_20 = 20 * x_history[20 - 1];
    let signal_strength_60 = 60 * x_history[60 - 1];
    let signal_strength_100 = 100 * x_history[100 - 1];
    let signal_strength_140 = 140 * x_history[140 - 1];
    let signal_strength_180 = 180 * x_history[180 - 1];
    let signal_strength_220 = 220 * x_history[220 - 1];

    println!(
        "{:?}",
        signal_strength_20
            + signal_strength_60
            + signal_strength_100
            + signal_strength_140
            + signal_strength_180
            + signal_strength_220
    );

    for (cycle, x) in x_history.iter().enumerate().take(240) {
        if cycle % 40 == 0 {
            println!();
        }

        let x_pos = (cycle as i32) % 40;

        if (x_pos - 1..=x_pos + 1).contains(x) {
            print!("#");
        } else {
            print!(".");
        }
    }
}
