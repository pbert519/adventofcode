use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let mut lines = BufReader::new(file).lines();

    let alogrithm = lines.next().unwrap().unwrap();
    lines.next(); // skip empty line
    let mut inital_image = lines.map(|x| {
        x.unwrap().chars().map(|c| if c == '#' {
            true
        } else {
            false
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    for i in 0..50 {
        inital_image = enhance(&inital_image, &alogrithm, (i%2)==1);
    }

    let count = inital_image.iter().flatten().filter(|x| **x ).count();
    println!("Light pixels {}", count);
    //19848 to high
    //20165 to high
}

fn print_image(enhanced_2: &Vec<Vec<bool>>) {
    for row in enhanced_2 {
        row.iter().for_each(|bit| {
            if *bit {
                print!("#")
            } else {
                print!(".")
            }
        });
        println!();
    }
}

fn enhance(image: &Vec<Vec<bool>>, algorithm: &String, init: bool) -> Vec<Vec<bool>> {
    let width = image[0].len()+2;
    let height = image.len()+2;
    let mut new_image = vec![vec![init; width]; height];

    for h in 0..height as i32 {
        for w in 0..width as i32 {
            let code = get_code(h,w,image,init);
            let value = if algorithm.as_bytes()[code as usize] as char == '#' {
                true
            } else {
                false
            };

            new_image[h as usize][w as usize] = value;
        }
    }

    new_image
}

fn get_code(h:i32,w:i32, image: &Vec<Vec<bool>>, init :bool) -> i32 {
    let mut code = Vec::new();
    code.push(get_pixel(h-1,w-1,image,init));
    code.push(get_pixel(h-1,w,image,init));
    code.push(get_pixel(h-1,w+1,image,init));
    code.push(get_pixel(h,w-1,image,init));
    code.push(get_pixel(h,w,image,init));
    code.push(get_pixel(h,w+1,image,init));
    code.push(get_pixel(h+1,w-1,image,init));
    code.push(get_pixel(h+1,w,image,init));
    code.push(get_pixel(h+1,w+1,image,init));
    code.iter().fold(0, |result, bit|
        (result << 1) ^ (*bit as i32)
    )
}

fn get_pixel(h:i32,w:i32, image: &Vec<Vec<bool>>, default:bool) -> bool {
    let max_h = image.len() as i32;
    let max_w = image[0].len() as i32;

    if h > 0 && h <= max_h && w > 0 && w <= max_w {
        image[(h-1) as usize][(w-1) as usize]
    } else {
        default
    }
}