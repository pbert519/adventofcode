use std::fs::File;
use std::io::{BufRead, BufReader};

// Rock     1   A   X   win
// Paper    2   B   Y   draw
// scissors 3   C   Z   lose

// 0pt lost
// 3pt draw
// 6pt won


fn score_simple(line: &str) -> i32{
    match line {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        _ => panic!()  
    }
}

fn map_to_response(line: &str) -> String {
    match line {
        "A X" => "A Z".to_string(),
        "A Y" => "A X".to_string(),
        "A Z" => "A Y".to_string(),
        "B X" => "B X".to_string(),
        "B Y" => "B Y".to_string(),
        "B Z" => "B Z".to_string(),
        "C X" => "C Y".to_string(),
        "C Y" => "C Z".to_string(),
        "C Z" => "C X".to_string(),
        _ => panic!()  
    }
}

fn main() {
    let file = File::open("input_test.txt").expect("Could not open file");
    let input = BufReader::new(file).lines();

    let score: i32 = input.map(|mut line| {
        let line = line.unwrap();

        score_simple(&line)
        
    }).sum();    

    println!("Score A would be: {}!", score);


    let file = File::open("input.txt").expect("Could not open file");
    let input = BufReader::new(file).lines();
    let score_b: i32 = input.map(|mut line| {
        let line = line.unwrap();
        map_to_response(&line)
    }).map(|line| {
        score_simple(&line)
    }).sum();


    println!("Score B would be: {}!", score_b);
}
