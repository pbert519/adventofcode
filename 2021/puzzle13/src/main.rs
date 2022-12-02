use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let lines = BufReader::new(file).lines();
    let mut dots = lines
        .filter_map(|x| {
            let x = x.unwrap();
            if !x.contains(',') {
                None
            } else {
                let mut splited = x.split(',').map(|c| c.parse::<i32>().unwrap());
                Some((splited.next().unwrap(), splited.next().unwrap()))
            }
        })
        .collect::<BTreeSet<(i32, i32)>>();

    let file = File::open("input.txt").expect("Could not open file");
    let lines = BufReader::new(file).lines();
    let folds = lines
        .filter_map(|x| {
            let x = x.unwrap();
            if !x.contains(',') && x.len() > 1 {
                let a = x.split('=').last().unwrap().parse::<i32>().unwrap();
                if x.contains('x') {
                    Some(Fold::X(a))
                } else {
                    Some(Fold::Y(a))
                }
            } else {
                None
            }
        })
        .collect::<Vec<Fold>>();

    println!("{:?}", dots);
    println!("{:?}", folds);

    for fold in folds {
        dots = dots
            .iter()
            .map(|dot| fold_dot(&fold, dot))
            .filter(|(a, b)| (*a >= 0) && (*b >= 0))
            .collect::<BTreeSet<(i32, i32)>>();

        println!("{}, {:?}, {:?}", dots.len(), fold, dots);
    }

    let mut array = vec![vec![' '; 40]; 6];
    for (x, y) in dots {
        array[y as usize][x as usize] = '#';
    }
    for row in array {
        println!("{:?}", row);
    }
}

fn fold_dot(fold: &Fold, dot: &(i32, i32)) -> (i32, i32) {
    let (mut x, mut y) = *dot;
    match fold {
        Fold::X(c) => {
            if x > *c {
                x = 2 * *c - x
            }
        }
        Fold::Y(c) => {
            if y > *c {
                y = 2 * *c - y
            }
        }
    }
    (x, y)
}
