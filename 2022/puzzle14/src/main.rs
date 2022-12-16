use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum Item {
    Rock,
    Sand,
}

fn main() {
    let sand_start = (500, 0);
    let mut x_min = 500;
    let mut x_max = 500;
    let mut y_max = 0;

    let mut map = BTreeMap::new();

    let file = File::open("input.txt").unwrap();
    BufReader::new(file).lines().for_each(|line| {
        let line = line.unwrap();
        line.split(" -> ")
            .map(|item| {
                let mut pos = item.split(',');
                let x = pos.next().unwrap().parse::<i32>().unwrap();
                let y = pos.next().unwrap().parse::<i32>().unwrap();

                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_max = y_max.max(y);

                (x, y)
            })
            .collect::<Vec<_>>()
            .windows(2)
            .for_each(|points| {
                let start = points[0];
                let stop = points[1];

                if start.0 == stop.0 {
                    // in y dir
                    let y_start = start.1.min(stop.1);
                    let y_stop = start.1.max(stop.1);
                    for y in y_start..=y_stop {
                        map.insert((start.0, y), Item::Rock);
                    }
                } else {
                    // in x dir
                    let x_start = start.0.min(stop.0);
                    let x_stop = start.0.max(stop.0);
                    for x in x_start..=x_stop {
                        map.insert((x, start.1), Item::Rock);
                    }
                }
            })
    });

    let map_b = map.clone();

    loop {
        // spawn a new sand
        let mut sand_pos = sand_start;
        if map.contains_key(&sand_pos) {
            panic!();
        }

        loop {
            if !map.contains_key(&(sand_pos.0, sand_pos.1 + 1)) {
                // down
                sand_pos = (sand_pos.0, sand_pos.1 + 1);
                if sand_pos.1 > y_max {
                    break;
                }
            } else if !map.contains_key(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                // down left
                sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
            } else if !map.contains_key(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
                // down right
                sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
            } else {
                // final sand position
                break;
            }
        }
        if sand_pos.1 > y_max {
            break;
        }
        map.insert(sand_pos, Item::Sand);
    }

    let stable_sand_count = map
        .iter()
        .filter(|(_, item)| match item {
            Item::Rock => false,
            Item::Sand => true,
        })
        .count();

    println!("Part 1: {}", stable_sand_count);

    // part b

    let mut map = map_b;
    loop {
        // spawn a new sand
        let mut sand_pos = sand_start;
        if map.contains_key(&sand_pos) {
            break;
        }

        loop {
            if sand_pos.1 == y_max + 1 {
                break;
            } else if !map.contains_key(&(sand_pos.0, sand_pos.1 + 1)) {
                // down
                sand_pos = (sand_pos.0, sand_pos.1 + 1);
            } else if !map.contains_key(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                // down left
                sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
            } else if !map.contains_key(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
                // down right
                sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
            } else {
                // final sand position
                break;
            }
        }
        map.insert(sand_pos, Item::Sand);
    }

    let stable_sand_count = map
        .iter()
        .filter(|(_, item)| match item {
            Item::Rock => false,
            Item::Sand => true,
        })
        .count();

    println!("Part 1: {}", stable_sand_count);
}
