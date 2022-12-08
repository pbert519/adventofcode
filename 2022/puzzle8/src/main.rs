use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let input = BufReader::new(file);

    let map = input.lines().fold(Vec::new(), |mut acc, line| {
        let line = line.unwrap();

        let row = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        acc.push(row);
        acc
    });

    // part 1
    let mut counter = 0;
    for row_index in 1..map.len() - 1 {
        for column_index in 1..map[0].len() - 1 {
            let height = map[row_index][column_index];

            let mut visible = false;

            // check nort
            let mut index = row_index - 1;
            while height > map[index][column_index] {
                //println!("\t Pos {}:{} {} is lower" , index, column_index, map[index][column_index] );

                if index == 0 {
                    break;
                } else {
                    index -= 1;
                }
            }
            if height > map[index][column_index] {
                visible = true;
            }

            // check south
            let mut index = row_index + 1;
            while height > map[index][column_index] {
                //println!("\t Pos {}:{} {} is lower" , index, column_index, map[index][column_index] );

                if index == map.len() - 1 {
                    break;
                } else {
                    index += 1;
                }
            }
            if height > map[index][column_index] {
                visible = true;
            }

            // check west
            let mut index = column_index - 1;
            while height > map[row_index][index] {
                //println!("\t Pos {}:{} {} is lower" , index, column_index, map[index][column_index] );

                if index == 0 {
                    break;
                } else {
                    index -= 1;
                }
            }
            if height > map[row_index][index] {
                visible = true;
            }

            // check east
            let mut index = column_index + 1;
            while height > map[row_index][index] {
                //println!("\t Pos {}:{} {} is lower" , index, column_index, map[index][column_index] );

                if index == map[0].len() - 1 {
                    break;
                } else {
                    index += 1;
                }
            }
            if height > map[row_index][index] {
                visible = true;
            }

            //println!(
            //    "Process {}:{} {} is visible: {}",
            //    row_index, column_index, height, visible
            //);
            if visible {
                counter += 1;
            }
        }
    }

    let visible_trees_out = map.len() * 2 + (map[0].len() - 2) * 2;
    println!(
        "Outer visible trees {}, inner visible trees {}, sum: {}",
        visible_trees_out,
        counter,
        visible_trees_out + counter
    );

    // part 2
    let mut highest_score = 0;
    for row_index in 1..map.len() - 1 {
        for column_index in 1..map[0].len() - 1 {
            let height = map[row_index][column_index];

            // check north
            let mut north_counter = 1;
            let mut index = row_index - 1;
            while height > map[index][column_index] {
                if index == 0 {
                    break;
                } else {
                    index -= 1;
                }
                north_counter += 1;
            }

            // check south
            let mut south_counter = 1;
            let mut index = row_index + 1;
            while height > map[index][column_index] {
                if index == map.len() - 1 {
                    break;
                } else {
                    index += 1;
                }
                south_counter += 1;
            }

            // check west
            let mut west_counter = 1;
            let mut index = column_index - 1;
            while height > map[row_index][index] {
                if index == 0 {
                    break;
                } else {
                    index -= 1;
                }
                west_counter += 1;
            }

            // check east
            let mut east_counter = 1;
            let mut index = column_index + 1;
            while height > map[row_index][index] {
                if index == map[0].len() - 1 {
                    break;
                } else {
                    index += 1;
                }

                east_counter += 1;
            }

            let scenic_score = north_counter * south_counter * west_counter * east_counter;
            //println!(
            //    "Process {}:{} {} is visible: {}",
            //    row_index, column_index, height, scenic_score
            //);
            //println!("{} * {}* {} * {}", north_counter, south_counter, west_counter,east_counter);
            if scenic_score > highest_score {
                highest_score = scenic_score;
            }
        }
    }

    println!("Highest scenic score {}", highest_score);
}
