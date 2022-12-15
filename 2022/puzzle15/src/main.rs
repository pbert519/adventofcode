use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
};

const Y_ROW: i64 = 2000000;
//const Y_ROW: i64 = 10;

fn main() {
    let file = File::open("input.txt").unwrap();

    let mut beacons = BTreeSet::new();

    let input = BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let mut items = line.split_whitespace();
            let s_x_str = items.nth(2).unwrap()[2..].to_string();
            let s_x = s_x_str[0..s_x_str.len() - 1].parse::<i64>().unwrap();
            let s_y_str = items.next().unwrap()[2..].to_string();
            let s_y = s_y_str[0..s_y_str.len() - 1].parse::<i64>().unwrap();

            let b_x_str = items.nth(4).unwrap()[2..].to_string();
            let b_x = b_x_str[0..b_x_str.len() - 1].parse::<i64>().unwrap();
            let b_y_str = items.next().unwrap()[2..].to_string();
            let b_y = b_y_str[0..b_y_str.len()].parse::<i64>().unwrap();

            //println!("Sensor at {},{} with beacon {},{}", s_x, s_y, b_x, b_y);

            beacons.insert((b_x, b_y));

            ((s_x, s_y), (b_x, b_y))
        })
        .collect::<Vec<_>>();

    let mut ranges = input.iter().fold(Vec::new(), |mut set, (sensor, beacon)| {
        let dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

        if (sensor.1 - dist..=sensor.1 + dist).contains(&Y_ROW) {
            let x_dist = dist - (sensor.1 - Y_ROW).abs();

            set.push(sensor.0 - x_dist..=sensor.0 + x_dist);
        }
        set
    });

    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut unique_ranges = Vec::new();
    unique_ranges.push(ranges.remove(0));

    for range in ranges {
        let last_unique = unique_ranges.last_mut().unwrap();
        // overlap
        if range.start() <= last_unique.end() {
            let end = range.end().max(last_unique.end());
            *last_unique = *last_unique.start()..=*end;
        } else {
            unique_ranges.push(range);
        }
    }

    println!("Ranges: {:?}", unique_ranges);

    println!(
        "Part 1: {:?}",
        *unique_ranges[0].end() - *unique_ranges[0].start()
    );

    // part 2

    let min_pos = 0;
    let max_pos = 4000000;

    for y_row in min_pos..max_pos {
        let mut ranges = input.iter().fold(Vec::new(), |mut set, (sensor, beacon)| {
            let dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

            if (sensor.1 - dist..=sensor.1 + dist).contains(&y_row) {
                let x_dist = dist - (sensor.1 - y_row).abs();

                set.push(sensor.0 - x_dist..=sensor.0 + x_dist);
            }
            set
        });

        ranges.sort_by(|a, b| a.start().cmp(b.start()));
        let mut unique_ranges = Vec::new();
        unique_ranges.push(ranges.remove(0));

        for range in ranges {
            let last_unique = unique_ranges.last_mut().unwrap();
            // overlap
            if range.start() <= last_unique.end() {
                let end = range.end().max(last_unique.end());
                *last_unique = *last_unique.start()..=*end;
            } else {
                unique_ranges.push(range);
            }
        }

        // check for a free spots
        for ur in &unique_ranges {
            if *ur.start() > min_pos || *ur.end() < max_pos {
                println!("Y: {}, Ranges: {:?}", y_row, unique_ranges)
            }
        }
    }

    // Extracted from ranges
    let x_result: i64 = 3138881;
    let y_result: i64 = 3364986;
    let result = x_result * 4000000 + y_result;

    println!("Part B {}", result);
}
