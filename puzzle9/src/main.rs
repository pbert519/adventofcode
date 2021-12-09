use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let input = BufReader::new(file).lines();

    let map = input
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|char| char.to_digit(10).expect("Could not parse digit"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    // extract min points
    let mut min_points: Vec<u32> = Vec::new();
    let mut basins: Vec<BTreeSet<(usize, usize)>> = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            let current = map[row][col];
            let mut adjacent_lower = false;
            if row > 0 && map[row - 1][col] <= current {
                adjacent_lower = true;
            }
            if col > 0 && map[row][col - 1] <= current {
                adjacent_lower = true;
            }
            if row < map.len() - 1 && map[row + 1][col] <= current {
                adjacent_lower = true;
            }
            if col < map[0].len() - 1 && map[row][col + 1] <= current {
                adjacent_lower = true;
            }
            if !adjacent_lower {
                // found a min_point
                min_points.push(current);
                // based on the min point, expand the basin
                let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();
                create_basin(&mut set, &map, row, col);
                basins.push(set);
            }
        }
    }
    println!("Local minimum points: {:?}", min_points);

    let score = min_points.iter().fold(0, |acc, p| acc + (p + 1));
    println!("Result is : {:?}", score);

    let mut basins_size = basins.iter().map(|b| b.len()).collect::<Vec<_>>();
    basins_size.sort_unstable();
    let score: usize = basins_size.iter().rev().take(3).product();
    println!("Basins: {:?}", basins);
    println!("Basins size: {:?}", basins_size);
    println!("Part2 score: {:?}", score);
}

fn create_basin(set: &mut BTreeSet<(usize, usize)>, map: &[Vec<u32>], row: usize, col: usize) {
    if map[row][col] == 9 || set.contains(&(row, col)) {
        return;
    }

    set.insert((row, col));

    if row > 0 {
        create_basin(set, map, row - 1, col);
    }
    if col > 0 {
        create_basin(set, map, row, col - 1);
    }
    if row < map.len() - 1 {
        create_basin(set, map, row + 1, col);
    }
    if col < map[0].len() - 1 {
        create_basin(set, map, row, col + 1);
    }
}
