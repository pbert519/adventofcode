#![feature(map_first_last)]
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

// data type for heap, sorted by min cost
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (i32, i32),
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let lines = BufReader::new(file).lines();

    let mut max = 0;
    let grid = lines
        .enumerate()
        .inspect(|_| max += 1)
        .map(|(row, line)| {
            line.expect("Could not read line")
                .chars()
                .map(|c| c.to_digit(10).expect("Could not parse value"))
                .enumerate()
                .map(|(col, energy)| ((row as i32, col as i32), energy as i32))
                .collect::<HashMap<_, _>>()
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    // increase grid for part 2
    let grid = grid
        .into_iter()
        .fold(HashMap::new(), |mut map, (pos, risk)| {
            for xcopy in 0..5i32 {
                for ycopy in 0..5i32 {
                    let mut new_risk = xcopy + ycopy + risk;
                    if new_risk > 9 {
                        new_risk %= 9;
                    }
                    map.insert((pos.0 + xcopy * (max), pos.1 + ycopy * (max)), new_risk);
                }
            }
            map
        });
    let max = max * 5;

    let start = (0, 0);
    let stop = (max - 1, max - 1);

    let mut distances = HashMap::new();
    distances.insert(start, 0i32);

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == stop {
            println!("Reached goal with cost: {}", cost);
            break;
        }

        if cost > *distances.get(&position).unwrap() {
            continue;
        }

        visit_neighbour(
            &mut heap,
            &grid,
            &mut distances,
            cost,
            (position.0 - 1, position.1),
            max,
        );
        visit_neighbour(
            &mut heap,
            &grid,
            &mut distances,
            cost,
            (position.0 + 1, position.1),
            max,
        );
        visit_neighbour(
            &mut heap,
            &grid,
            &mut distances,
            cost,
            (position.0, position.1 - 1),
            max,
        );
        visit_neighbour(
            &mut heap,
            &grid,
            &mut distances,
            cost,
            (position.0, position.1 + 1),
            max,
        );
    }
}

fn visit_neighbour(
    heap: &mut BinaryHeap<State>,
    grid: &HashMap<(i32, i32), i32>,
    distances: &mut HashMap<(i32, i32), i32>,
    risk: i32,
    pos: (i32, i32),
    max: i32,
) {
    if !(pos.0 < 0 || pos.1 < 0 || pos.0 >= max || pos.1 >= max) {
        let edge_cost = grid.get(&pos).unwrap();
        let next = State {
            cost: risk + edge_cost,
            position: pos,
        };
        if let Some(e) = distances.get_mut(&pos) {
            if *e > next.cost {
                *e = next.cost;
                heap.push(next);
            }
        } else {
            heap.push(next);
            distances.insert(pos, next.cost);
        }
    }
}
