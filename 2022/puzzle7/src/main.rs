use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let input = BufReader::new(file).lines();

    let mut folders = BTreeMap::new();
    let mut active_folder = Vec::new();

    for line in input {
        let line = line.unwrap();

        if line.starts_with("$ cd") {
            // a cd command
            if line.ends_with("..") {
                active_folder.pop();
            } else {
                let new_dir = line.split(' ').nth(2).unwrap().to_string();
                let path = String::from_iter(active_folder.clone().into_iter());
                let new_dir = path + "/" + &new_dir;
                active_folder.push(new_dir);
            }
        } else if line.starts_with(|c: char| c.is_ascii_digit()) {
            // a file size
            let size: i32 = line.split(' ').next().unwrap().parse().unwrap();
            for folder in &active_folder {
                *folders.entry(folder.clone()).or_insert(0) += size;
            }
        }
    }

    let mut result = 0;
    for size in folders.values() {
        if *size <= 100000 {
            result += size;
        }
    }
    println!("Result 1 {}", result);

    // part b

    let total_space = 70000000;
    let update_space = 30000000;
    let used_space = folders["//"];

    let free_up_space_min = update_space - (total_space - used_space);

    let smallest_folder_size = folders
        .iter()
        .filter_map(|(_, &size)| {
            if size > free_up_space_min {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    println!("Result 2 {}", smallest_folder_size);
}
