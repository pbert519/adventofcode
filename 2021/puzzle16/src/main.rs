use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let line = BufReader::new(file).lines().next().unwrap().unwrap();
    let mut input = line
        .chars()
        .map(|char| to_binary(char).chars())
        .flatten()
        .collect::<String>();

    let (sum_of_versions, value) = decode_package(&mut input);

    println!("(Part1) Sum of all version numbers {:?}", sum_of_versions);
    println!("(Part2) Result of all operators {:?}", value)
}

fn decode_package(input: &mut String) -> (i64, i64) {
    let version = i64::from_str_radix(input.drain(0..3).as_str(), 2).unwrap();
    let mut sum_of_version = version;

    let type_id = i64::from_str_radix(input.drain(0..3).as_str(), 2).unwrap();
    if type_id == 4 {
        // literal value
        let mut value_str = String::new();
        while input.drain(0..1).as_str() == "1" {
            value_str.push_str(input.drain(0..4).as_str());
        }
        value_str.push_str(input.drain(0..4).as_str());
        (sum_of_version, i64::from_str_radix(&value_str, 2).unwrap())
    } else {
        // operator package
        let length_type = i64::from_str_radix(input.drain(0..1).as_str(), 2).unwrap();
        let mut values = Vec::new();
        if length_type == 0 {
            // total length of bits
            let bit_length = usize::from_str_radix(input.drain(0..15).as_str(), 2).unwrap();
            let mut payload = input.drain(0..bit_length).as_str().to_string();
            while payload.len() > 3 {
                let (new_version, new_value) = decode_package(&mut payload);
                sum_of_version += new_version;
                values.push(new_value);
            }
        } else {
            // number of sub packets
            let package_count = usize::from_str_radix(input.drain(0..11).as_str(), 2).unwrap();
            for _ in 0..package_count {
                let (new_version, new_value) = decode_package(input);
                sum_of_version += new_version;
                values.push(new_value);
            }
        }
        // execute operator and return
        (sum_of_version, calc_operator(type_id, &values))
    }
}

fn calc_operator(type_id: i64, values: &[i64]) -> i64 {
    if type_id == 0 {
        values.iter().sum()
    } else if type_id == 1 {
        values.iter().product()
    } else if type_id == 2 {
        *values.iter().min().unwrap()
    } else if type_id == 3 {
        *values.iter().max().unwrap()
    } else if type_id == 5 {
        let bool: bool = values[0] > values[1];
        bool as i64
    } else if type_id == 6 {
        let bool: bool = values[0] < values[1];
        bool as i64
    } else if type_id == 7 {
        let bool: bool = values[0] == values[1];
        bool as i64
    } else {
        panic!("type id not valid")
    }
}
