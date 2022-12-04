use std::{fs::File, io::{BufReader, BufRead}};



fn main() {
    let file = File::open("input.txt").expect("Could not read file");
    let lines = BufReader::new(file).lines();

    let count = lines.filter(|line| {
        let line = line.as_ref().unwrap();

        let mut elves = line.split(",").map(|range| {
            let mut range = range.split("-").map(|number| number.parse::<i32>().unwrap());
            let start = range.next().unwrap();
            let stop = range.next().unwrap();
            start..=stop
        });
        let elf_a = elves.next().unwrap();
        let elf_b = elves.next().unwrap();        

        let a_contains_b = elf_a.start() <= elf_b.start() && elf_a.end() >= elf_b.end();
        let b_contains_a = elf_b.start() <= elf_a.start() && elf_b.end() >= elf_a.end();


        a_contains_b || b_contains_a
    }).count();    


    println!("Number of assigned pairs with one rage fully contain the other {}", count);

    // part b

    let file = File::open("input.txt").expect("Could not read file");
    let lines = BufReader::new(file).lines();

    let count = lines.filter(|line| {
        let line = line.as_ref().unwrap();

        let mut elves = line.split(",").map(|range| {
            let mut range = range.split("-").map(|number| number.parse::<i32>().unwrap());
            let start = range.next().unwrap();
            let stop = range.next().unwrap();
            start..=stop
        });
        let elf_a = elves.next().unwrap();
        let elf_b = elves.next().unwrap();        

        // range 1 is lower than range 2
        let overlap_1 = elf_a.start() <= elf_b.start() && elf_b.start() <= elf_a.end();
        // rnage 2 is lower than range 1
        let overlap_2 = elf_b.start() <= elf_a.start() && elf_a.start() <= elf_b.end();

        overlap_1 || overlap_2
    }).count();    


    println!("Number of assigned pairs which overlap {}", count);

}
