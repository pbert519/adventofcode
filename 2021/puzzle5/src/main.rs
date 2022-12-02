use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Line {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn extract_position(raw: &str) -> (i32, i32) {
    let parsed = raw.split(',')
        .map(|x| x.parse::<i32>().expect("Could not parse"))
        .collect::<Vec<_>>();
    (parsed[0],parsed[1])
}

fn main() {

    let file = File::open("input.txt").expect("Could not open file");
    let input = BufReader::new(file).lines();
    let lines = input.map(|line| {
        if let Ok(l) = line {
            let mut components = l.split_whitespace();
            let (x1, y1) = extract_position(components.next().unwrap());
            components.next(); // skip arrow
            let (x2, y2) = extract_position(components.next().unwrap());
            Line {
                x1,
                x2,
                y1,
                y2
            }
        } else {
            panic!()
        }
    }).collect::<Vec<Line>>();

    let mut occupied: HashMap<(i32, i32), i32> = HashMap::new();
    for f in lines {
        println!("{:?}", f);
        if f.x1 == f.x2 {
            let (a, b) = sort(f.y1, f.y2);
            for y in a..b + 1 {
                let point = (f.x1, y);
                *occupied.entry(point).or_insert(0) += 1;
            }
        } else if f.y1 == f.y2 {
            let (a, b) = sort(f.x1, f.x2);
            for x in a..b + 1 {
                let point = (x, f.y1);
                *occupied.entry(point).or_insert(0) += 1;
            }
        } else if (f.x1 - f.x2).abs() == (f.y1 - f.y2).abs() {
            for p in get_points_from_line(& f) {
                *occupied.entry(p).or_insert(0) += 1;
            }

        }

        //println!("{:?}", occupied);
    }
    let dangerous_places_count = occupied.iter().filter(|(_, key)| **key > 1).count();
    println!("{:?}", dangerous_places_count);

}

fn get_points_from_line(line : & Line) -> Vec<(i32, i32)>{
    let steps = (line.x1 - line.x2).abs();
    let x_step = (line.x2 - line.x1) / steps;
    let y_step = (line.y2 - line.y1) / steps;
    println!("{:?} {:?} {:?}", steps,x_step,y_step);


    let mut result = Vec::with_capacity(steps as usize);
    for step in 0..steps+1 {
          result.push((line.x1+x_step*step, line.y1+y_step*step));
    }
    result
}

fn sort(a:i32,b:i32) -> (i32, i32) {
    if a > b {
        (b,a)
    } else {
        (a,b)
    }
}
