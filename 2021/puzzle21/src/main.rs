use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let mut lines = BufReader::new(file).lines();

    let mut player1_pos = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap()
        - 1;
    let mut player2_pos = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap()
        - 1;

    println!("Start-positions {}, {}", player1_pos, player2_pos);

    let mut dice_rolls = 0u32;
    let mut player1_score = 0u32;
    let mut player2_score = 0u32;

    while player1_score < 1000 && player2_score < 1000 {
        player1_pos = (player1_pos + roll_dice_three(&mut dice_rolls)) % 10;
        player1_score += player1_pos + 1;

        if player1_score >= 1000 {
            break;
        }

        player2_pos = (player2_pos + roll_dice_three(&mut dice_rolls)) % 10;
        player2_score += player2_pos + 1;

        println!(
            "Player1: {}, {}; Player 2: {}, {}",
            player1_pos, player1_score, player2_pos, player2_score
        );
    }
    println!(
        "Finished game with scores {}, {} after {} dice rolls",
        player1_score, player2_score, dice_rolls
    );



}

fn roll_dice_three(dice_rolls: &mut u32) -> u32 {
    let mut res = 0u32;
    for _ in 0..3 {
        res += (*dice_rolls % 100u32) + 1;
        *dice_rolls += 1;
    }
    res
}
