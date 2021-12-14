use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let mut lines = BufReader::new(file).lines();

    // load
    let input = lines.next().unwrap().unwrap();
    let mut polymer =
        input
            .chars()
            .zip(input.chars().skip(1))
            .fold(HashMap::new(), |mut acc, (a, b)| {
                let mut pair = a.to_string();
                pair.push(b);
                *acc.entry(pair).or_insert(0) += 1;
                acc
            });
    //println!("{:?}", polymer);

    let rules = lines
        .skip(1)
        .map(|line| {
            let line = line.unwrap();
            let mut elements = line.split(" -> ");
            (
                elements.next().unwrap().to_string(),
                elements.next().unwrap().to_string(),
            )
        })
        .collect::<HashMap<_, _>>();
    //println!("{:?}", rules);

    for _ in 0..40 {
        polymer = polymer
            .iter()
            .fold(HashMap::new(), |mut acc, (pair, count)| {
                if rules.contains_key(pair) {
                    let mut new_pair_1 = pair[0..1].to_string();
                    new_pair_1.push_str(rules.get(pair).unwrap());
                    *acc.entry(new_pair_1).or_insert(0i64) += count;

                    let mut new_pair_2 = rules.get(pair).unwrap().clone();
                    new_pair_2.push_str(&pair[1..2]);
                    *acc.entry(new_pair_2).or_insert(0i64) += count;
                } else {
                    acc.entry(pair.clone()).or_insert(*count);
                }
                acc
            });
        //println!("{:?}", polymer);
    }

    // now analyse the generated polymers
    let mut count = polymer
        .iter()
        .fold(HashMap::new(), |mut acc, (pair, count)| {
            pair.chars()
                .skip(1)
                .for_each(|char| *acc.entry(char).or_insert(0) += count);
            acc
        });
    *count.entry(input.chars().next().unwrap()).or_insert(0) += 1;

    let mut sorted = count.iter().collect::<Vec<_>>();
    sorted.sort_by(|a, b| a.1.cmp(b.1));
    let score = sorted.last().unwrap().1 - sorted.first().unwrap().1;

    println!("{:?}", score);
}
