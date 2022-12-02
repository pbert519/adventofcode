use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Seek};
use ndarray::{Array, Ix3, s};

const BITWIDTH: usize = 5;

fn to_u32(slice: &[bool]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

fn main() {
    part1_new();

    //part1();

    //part2();
}

fn part2() {
    let file = File::open("input.txt").expect("Could not open file");
    let bufread = BufReader::new(file).lines();
    let parsed = convert_to_bool(bufread);

    // find oxygen rating
    let mut oxygen = parsed.clone();
    let mut iteration = 0;
    while oxygen.len() > 1 && iteration < BITWIDTH {
        let gamma = extract_common_value(transpose(oxygen.clone()));
        oxygen = oxygen
            .iter()
            .filter(|v| v[iteration] == gamma[iteration])
            .cloned()
            .collect::<Vec<Vec<bool>>>();
        iteration += 1;
    }
    assert_eq!(oxygen.len(), 1);
    let oxygen_dec = to_u32(&oxygen[0]);
    println!("Oxygen rating is: {}", oxygen_dec);

    // find co2 rating
    let mut co2 = parsed;
    let mut iteration = 0;
    while co2.len() > 1 && iteration < BITWIDTH {
        let gamma = extract_common_value(transpose(co2.clone()));
        let epsilon = revert_common_value(&gamma);
        co2 = co2
            .iter()
            .filter(|v| v[iteration] == epsilon[iteration])
            .cloned()
            .collect::<Vec<Vec<bool>>>();
        iteration += 1;
    }
    assert_eq!(co2.len(), 1);
    let co2_dec = to_u32(&co2[0]);
    println!("CO2 rating is: {}", co2_dec);
    println!("Life support rating is {}", co2_dec * oxygen_dec);
}

fn part1() {
    let file = File::open("input.txt").expect("Could not open file");
    let bufread = BufReader::new(file).lines();
    let parsed = convert_to_bool(bufread);

    let input = transpose(parsed);
    let gamma = extract_common_value(input);
    let epsilon = revert_common_value(&gamma);
    let gamma_dec = to_u32(&gamma);
    let epsilon_dec = to_u32(&epsilon);
    let power_dec = gamma_dec * epsilon_dec;
    println!(
        "Gamma: {:?}, Epsilon: {:?}, Power consumption: {:?}",
        gamma_dec, epsilon_dec, power_dec
    );
}

fn revert_common_value(gamma: &[bool]) -> Vec<bool> {
    gamma.iter().map(|x| !x).collect::<Vec<bool>>()
}

fn transpose(input: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let init = vec![Vec::with_capacity(1000); BITWIDTH];
    input.iter().fold(init, |mut acc, input| {
        input.iter().enumerate().for_each(|(n, v)| acc[n].push(*v));
        acc
    })
}
fn convert_to_bool(bufread: Lines<BufReader<File>>) -> Vec<Vec<bool>> {
    bufread
        .map(|input| {
            input
                .expect("Error reading input")
                .chars()
                .map(|c| c == '1')
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>()
}

fn extract_common_value(input: Vec<Vec<bool>>) -> Vec<bool> {
    input
        .iter()
        .map(|v| v.iter().partition(|b| **b))
        .map(|(t, f): (Vec<bool>, Vec<bool>)| t.len() >= f.len())
        .collect::<Vec<bool>>()
}


fn part1_new() {
    let file = File::open("input_test.txt").expect("Could not open file");
    let reader = BufReader::new(file).lines();
    let values =     reader
        .map(|input| {
            input
                .expect("Error reading input")
                .chars()
                .map(|c| c.to_digit(10).expect("Could not convert"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();
    let rows = values.len();
    let columns = values[0].len();
    let mut a = Array::<u32,_>::from_elem((rows,columns), 0);
    values.iter().enumerate().for_each(|(row, v)|
        v.iter().enumerate().for_each(|(col, elem)|
            a[[row,col]] = *elem
        )
    );




    let slice = a.slice(s![.., 0]);
    println!("{:?}", slice);


}
