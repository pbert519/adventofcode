use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let input = BufReader::new(file).lines();

    let mut elves_with_food: Vec<i32> = vec![0; 1];
    

    input.for_each(|food_item| {
        if let Ok(calories) = food_item.expect("Could not read string").parse::<i32>() {
            let last_index = elves_with_food.len() - 1;
            elves_with_food[last_index] += calories;
        } else {
            elves_with_food.push(0);
        }

    });

    elves_with_food.sort();


    println!("Elves with food: {:?}", elves_with_food);

    let most_calories = elves_with_food.iter().max().expect("no elves?!");
    println!("Elf with the most food has {} calories", most_calories);

    elves_with_food.reverse();
    let top_three_elves_calories: i32 = elves_with_food[0..3].iter().sum();
    println!("Top three elves carry {} calories", top_three_elves_calories);


}
