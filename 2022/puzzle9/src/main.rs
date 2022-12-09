use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let input = BufReader::new(file).lines();

    let mut tail_pos = BTreeSet::new();
    tail_pos.insert((0, 0));

    // .0 is right positive
    // .1 is up positive
    let mut head = (0, 0);
    let mut tail = (0, 0);

    for line in input {
        let line = line.unwrap();

        let dir = line.chars().next().unwrap();
        let steps = line.split(' ').nth(1).unwrap().parse::<i32>().unwrap();

        for _ in 0..steps {
            match dir {
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                'R' => head.0 += 1,
                'L' => head.0 -= 1,
                _ => panic!("invalid direction"),
            }

            // follow with tail
            let diff = (head.0 - tail.0, head.1 - tail.1);

            if diff.0 > 1 {
                tail.0 += 1;
                if diff.1 != 0 {
                    tail.1 += diff.1
                }
            } else if diff.0 < -1 {
                tail.0 -= 1;
                if diff.1 != 0 {
                    tail.1 += diff.1
                }
            } else if diff.1 > 1 {
                tail.1 += 1;
                if diff.0 != 0 {
                    tail.0 += diff.0
                }
            } else if diff.1 < -1 {
                tail.1 -= 1;
                if diff.0 != 0 {
                    tail.0 += diff.0
                }
            }

            // store tail pos
            tail_pos.insert(tail);
        }
    }

    println!("Part 1: Number of tail pos: {}", tail_pos.len());

    // part 2
    let file = File::open("input.txt").unwrap();
    let input = BufReader::new(file).lines();

    let mut tail_pos = BTreeSet::new();
    tail_pos.insert((0, 0));

    // .0 is right positive
    // .1 is up positive
    let mut knots = vec![(0, 0); 10];

    for line in input {
        let line = line.unwrap();

        let dir = line.chars().next().unwrap();
        let steps = line.split(' ').nth(1).unwrap().parse::<i32>().unwrap();

        for _ in 0..steps {
            match dir {
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                'R' => knots[0].0 += 1,
                'L' => knots[0].0 -= 1,
                _ => panic!("invalid direction"),
            }

            for i in 1..knots.len() {
                let head = knots[i - 1];
                let mut tail = knots[i];

                // follow with tail
                let diff: (i32, i32) = (head.0 - tail.0, head.1 - tail.1);

                if diff.0 > 1 {
                    tail.0 += 1;
                    if diff.1 > 0 {
                        tail.1 += 1
                    } else if diff.1 < 0 {
                        tail.1 -= 1
                    }
                } else if diff.0 < -1 {
                    tail.0 -= 1;
                    if diff.1 > 0 {
                        tail.1 += 1
                    } else if diff.1 < 0 {
                        tail.1 -= 1
                    }
                } else if diff.1 > 1 {
                    tail.1 += 1;
                    if diff.0 > 0 {
                        tail.0 += 1
                    } else if diff.0 < 0 {
                        tail.0 -= 1
                    }
                } else if diff.1 < -1 {
                    tail.1 -= 1;
                    if diff.0 > 0 {
                        tail.0 += 1
                    } else if diff.0 < 0 {
                        tail.0 -= 1
                    }
                }

                knots[i] = tail;
            }

            // store tail pos
            tail_pos.insert(knots[9]);
        }
    }

    println!("Part 2: Number of tail pos: {}", tail_pos.len());
}
