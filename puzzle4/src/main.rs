use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use itertools::{all, Itertools};
#[derive(Copy,Clone)]
struct Number {
    marked: bool,
    number: u32
}
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.marked {
            write!(f,"({:2})",self.number)
        } else {
            write!(f," {:2} ",self.number)
        }
    }
}
struct Board {
    numbers : [Number;25],
    won: bool
}
impl Board {
    fn mark_number(&mut self, number: u32){
        for elem in &mut self.numbers {
            if number == elem.number {
                elem.marked = true;
            }
        }
    }
    fn check_win(&self) -> bool {
        // rows:
        let mut win = false;
        for row in 0..5 {
            win |= self.numbers[row*5..row*5+5].iter().filter(|x| !x.marked).count() == 0
        }
        // columns
        for col in 0..5 {
            let mut is_marked = true;
            for row in 0..5 {
                if !self.numbers[row*5+col].marked {
                    is_marked = false;
                }
            }
            win |= is_marked;
        }
        win
    }
    fn calculate_unmarked_sum(&self) -> u32 {
        self.numbers.iter().filter(|x| !x.marked).fold(0, |acc, x| acc + x.number)
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..5 {
            for y in 0..5 {
                write!(f, "{}", self.numbers[x*5+y]);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}





fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let mut input = BufReader::new(file).lines();
    // extract drawn numbers
    let numbers = input.next().unwrap().expect("Could not read first line").split(',')
        .map(|x| x.parse::<u32>().expect("Could not parse"))
        .collect::<Vec<u32>>();
    println!("{:?}", numbers);

    // load boards
    let mut boards : Vec<Board> = Vec::new();
    while let Some(Ok(_)) = input.next() {
        let mut board: Board = Board { numbers: [Number{ marked: false, number: 0 }; 25], won: false };
        for i in 0..5 {
            let mut row = input.next().unwrap().unwrap().split_whitespace().map(|c| c.parse::<u32>().unwrap()).enumerate().for_each(|(n,v)|
                board.numbers[i*5+n] = Number{ marked: false, number: v }
            );
        }
        boards.push(board);
    }

    for number in numbers {
        println!("Draw number: {}", number);
        for board in &mut boards {
            board.mark_number(number);
            println!("{}", board);

            // check win
            if board.check_win() {
                //let score = board.calculate_unmarked_sum() * number;
                //println!("We have a winner, the score is {}!", score);
                board.won = true;

            }

        }
        // remove board which have won
        if boards.len() == 1 && boards[0].won {
            let score = boards[0].calculate_unmarked_sum() * number;
            println!("Only one board left, the score is {}!", score);
            return;
        }
        boards = boards.into_iter().filter(|b|!b.won).collect();

    }

}
