use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let line = BufReader::new(file).lines().next().unwrap().unwrap();
    // target area: x=20..30, y=-10..-5
    let re = Regex::new(r"^[\w|\s]*: x=([-+]?\d*)..([-+]?\d*), y=([-+]?\d*)..([-+]?\d*)")
        .expect("Regex expression not valid");
    let matches = re.captures_iter(&*line).next().unwrap();
    let x_min = matches[1].parse::<i32>().unwrap();
    let x_max = matches[2].parse::<i32>().unwrap();
    let y_min = matches[3].parse::<i32>().unwrap();
    let y_max = matches[4].parse::<i32>().unwrap();

    // get max and minimal velocity in x direction
    let max_x_velocity = x_max;
    let min_x_velocity = {
        // min_x_velocity*(min_x_velocity+1)/2 = min_distance -> unfold to (min_x_velocity-1)+(min_x_velocity-2)+...
        // min_x_velocity*(min_x_velocity+1) - 2*min_distance = 0
        // min_x_velocity² + in_x_velocity) - 2*min_distance = 0
        // pq equation
        let p = 1 as f64;
        let q = -2.0 * (x_min as f64);
        let x1 = -1.0 * (p / 2.0) + ((p / 2.0).powf(2.0) - q).sqrt();
        let x2 = -1.0 * (p / 2.0) - ((p / 2.0).powf(2.0) - q).sqrt();
        x1.max(x2).ceil() as i32
    };

    let min_y_velocity = y_min;
    let max_y_velocity = 200;

    println!("Target x: {}..{}, y: {}..{}", x_min, x_max, y_min, y_max);
    println!("x_velocity range: {:?}..{}", min_x_velocity, max_x_velocity);
    println!("y_velocity range: {:?}..{}", min_y_velocity, max_y_velocity);

    // brute force it
    // alternative starting with y_min and increase till overshoot
    let mut valid_count = 0;
    let mut vel_max = (0, 0);
    let mut find_y_max = 0;
    for x_velocity in min_x_velocity..=max_x_velocity {
        for y_velocity in min_y_velocity..200 {
            let mut pos = (0, 0);
            let vel = (x_velocity, y_velocity);
            let mut step = 0;

            let mut l_find_y_max = 0;
            while (pos.0 + (vel.0 - step).max(0)) <= x_max && (pos.1 + vel.1 - step) >= y_min {
                pos.0 += (vel.0 - step).max(0);
                pos.1 += vel.1 - step;
                step += 1;
                if l_find_y_max < pos.1 {
                    l_find_y_max = pos.1;
                }
            }
            if pos.0 <= x_max && pos.0 >= x_min && pos.1 >= y_min && pos.1 <= y_max {
                valid_count += 1;
                if l_find_y_max > find_y_max {
                    find_y_max = l_find_y_max;
                    vel_max = vel;
                }
            }
        }
    }
    println!(
        "Trick shot with velocity:  {:?} and y_max: {}",
        vel_max, find_y_max
    );
    println!("Number of possible solutions:  {:?}", valid_count);
}
