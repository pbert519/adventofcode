use std::{fs::File, io::{BufReader, BufRead}, collections::BTreeSet};

fn main() {
    
    let file = File::open("input.txt").unwrap();
    let mut input = BufReader::new(file).lines();

    // 109 106 113 106 112 113 109 103 98

    while let Some(input) = input.next() {
        if let Some((pos, _)) = input.unwrap().as_bytes().windows(4).enumerate().find(|(_, input)| {
            let set = input.iter().collect::<BTreeSet<_>>();
            set.len() == 4
        }) {
            println!("Marker at pos {}", pos+4);
        } else {
            panic!("No marker found")
        }     
    }


    // part b

    let file = File::open("input.txt").unwrap();
    let mut input = BufReader::new(file).lines();

    // 109 106 113 106 112 113 109 103 98

    while let Some(input) = input.next() {
        if let Some((pos, _)) = input.unwrap().as_bytes().windows(14).enumerate().find(|(_, input)| {
            let set = input.iter().collect::<BTreeSet<_>>();
            set.len() == 14
        }) {
            println!("Marker at pos {}", pos+14);
        } else {
            panic!("No marker found")
        }     
    }

}
