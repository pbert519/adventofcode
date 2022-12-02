use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let lines = BufReader::new(file).lines();

    let mut grid = lines
        .enumerate()
        .map(|(row, line)| {
            line.expect("Could not read line")
                .chars()
                .map(|c| c.to_digit(10).expect("Could not parse value"))
                .enumerate()
                .map(|(col, energy)| ((row as i32, col as i32), energy))
                .collect::<BTreeMap<_, _>>()
        })
        .flatten()
        .collect::<BTreeMap<_, _>>();

    // increase energy
    let mut flashes: i32 = 0;
    for step in 0.. {
        let mut flash: Vec<(i32, i32)> = Vec::new();
        for (pos, energy) in &mut grid {
            *energy += 1;
            if *energy > 9 {
                flash.push(*pos);
            }
        }
        // flash octopus
        let mut new_flashes = 0;
        for (row, col) in flash {
            new_flashes += flash_it(&mut grid, row, col);
        }
        if new_flashes == grid.len() as i32 {
            println!("Found a syncronized flashing at step {}", step + 1);
            break;
        }

        flashes += new_flashes;
        println!("{:?}", flashes);
    }
}

fn flash_it(grid: &mut BTreeMap<(i32, i32), u32>, row: i32, col: i32) -> i32 {
    if let Some(energy) = grid.get_mut(&(row, col)) {
        let mut flashes = 0;
        if *energy > 0 {
            *energy += 1;
            if *energy > 9 {
                *energy = 0;
                flashes += 1;
                flashes += flash_it(grid, row - 1, col - 1);
                flashes += flash_it(grid, row - 1, col);
                flashes += flash_it(grid, row - 1, col + 1);
                flashes += flash_it(grid, row, col - 1);
                flashes += flash_it(grid, row, col + 1);
                flashes += flash_it(grid, row + 1, col - 1);
                flashes += flash_it(grid, row + 1, col);
                flashes += flash_it(grid, row + 1, col + 1);
            }
        }
        flashes
    } else {
        0
    }
}
