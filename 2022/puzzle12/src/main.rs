use pathfinding::prelude::dijkstra;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, map: &Vec<Vec<u32>>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut next_pos = Vec::new();

        // -1, 0
        if x > 0 && map[x][y] + 1 >= map[x - 1][y] {
            next_pos.push(Pos(x - 1, y));
        }
        // 0, -1
        if y > 0 && map[x][y] + 1 >= map[x][y - 1] {
            next_pos.push(Pos(x, y - 1));
        }
        // 0, +1
        if y < map[0].len() - 1 && map[x][y] + 1 >= map[x][y + 1] {
            next_pos.push(Pos(x, y + 1));
        }
        // 1, 0
        if x < map.len() - 1 && map[x][y] + 1 >= map[x + 1][y] {
            next_pos.push(Pos(x + 1, y));
        }
        next_pos.into_iter().map(|p| (p, 1)).collect()
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let input = BufReader::new(file);

    let mut lowest_pos = Vec::new();
    let mut start = Pos(0, 0);
    let mut end = Pos(0, 0);

    let map = input
        .lines()
        .enumerate()
        .fold(Vec::new(), |mut acc, (row, line)| {
            let line = line.unwrap();

            let row = line
                .chars()
                .enumerate()
                .map(|(column, c)| {
                    if c == 'S' {
                        start = Pos(row, column);
                        0
                    } else if c == 'E' {
                        end = Pos(row, column);
                        25
                    } else {
                        if c == 'a' {
                            lowest_pos.push(Pos(row, column));
                        }
                        (c as u32) - 97
                    }
                })
                .collect::<Vec<_>>();

            acc.push(row);
            acc
        });
    lowest_pos.push(start.clone());

    let result = dijkstra(&start, |p| p.successors(&map), |p| *p == end);

    println!("Part 1: {}", result.unwrap().1);

    // part 2

    let mut possible_paths = Vec::new();
    for start in lowest_pos {
        let result = dijkstra(&start, |p| p.successors(&map), |p| *p == end);
        possible_paths.push(result);
    }

    let result_b = possible_paths
        .into_iter()
        .filter_map(|m| {
            if let Some((_, steps)) = m {
                Some(steps)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    println!("Part 2: {}", result_b);
}
