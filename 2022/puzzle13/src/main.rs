use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum PaketData {
    Int(i32),
    List(Vec<PaketData>),
}
fn compare_list(a: &Vec<PaketData>, b: &Vec<PaketData>) -> Option<std::cmp::Ordering> {
    for ai in 0..a.len() {
        // a is longer than b
        if b.len() < ai + 1 {
            return Some(std::cmp::Ordering::Greater);
        }
        let c = a[ai].cmp(&b[ai]);
        if c.is_ne() {
            return Some(c);
        }
    }
    if b.len() == a.len() {
        Some(std::cmp::Ordering::Equal)
    } else {
        Some(std::cmp::Ordering::Less)
    }
}

impl PartialOrd for PaketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            PaketData::Int(l_i) => match other {
                PaketData::Int(r_i) => l_i.partial_cmp(r_i),
                PaketData::List(r_list) => compare_list(&vec![PaketData::Int(*l_i)], r_list),
            },
            PaketData::List(l_list) => match other {
                PaketData::Int(r_i) => compare_list(l_list, &vec![PaketData::Int(*r_i)]),
                PaketData::List(r_list) => compare_list(l_list, r_list),
            },
        }
    }
}
impl Ord for PaketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_list(it: &mut impl Iterator<Item = char>) -> PaketData {
    let mut paket = Vec::new();

    let mut number_str = String::new();

    while let Some(c) = it.next() {
        match c {
            '[' => {
                let item = parse_list(it);
                paket.push(item);
            }
            ']' => {
                if !number_str.is_empty() {
                    let number = number_str.parse::<i32>().unwrap();
                    paket.push(PaketData::Int(number));
                    number_str.clear();
                }
                return PaketData::List(paket);
            }
            '0'..='9' => {
                number_str.push(c);
            }
            ',' => {
                if !number_str.is_empty() {
                    let number = number_str.parse::<i32>().unwrap();
                    paket.push(PaketData::Int(number));
                    number_str.clear();
                }
            }
            _ => {}
        }
    }

    panic!()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let input = BufReader::new(file)
        .lines()
        .filter_map(|l| {
            let line = l.unwrap();
            if line.is_empty() {
                None
            } else {
                Some(line)
            }
        })
        .map(|line| parse_list(&mut line.chars().peekable().skip(1)))
        .collect::<Vec<_>>();

    let mut part_b_input = input.clone();

    let right_order_count: usize = input
        .chunks(2)
        .enumerate()
        .flat_map(|(index, input)| {
            let left = &&input[0];
            let right = &&input[1];

            match left.cmp(right) {
                std::cmp::Ordering::Less => Some(index + 1),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => None,
            }
        })
        .sum();

    println!("Part 1: {}", right_order_count);

    // part b

    let divider1 = PaketData::List(vec![PaketData::List(vec![PaketData::Int(2)])]);
    let divider2 = PaketData::List(vec![PaketData::List(vec![PaketData::Int(6)])]);

    part_b_input.push(divider1.clone());
    part_b_input.push(divider2.clone());

    part_b_input.sort();

    let divider1_index = part_b_input.binary_search(&divider1).unwrap() + 1;
    let divider2_index = part_b_input.binary_search(&divider2).unwrap() + 1;

    println!("Part 2: {}", divider1_index * divider2_index);
}
