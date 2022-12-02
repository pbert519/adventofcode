use std::fs::File;
use std::io::{BufRead, BufReader};

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Command {
    fn from_string(raw: &str) -> Self {
        let mut split = raw.split_whitespace();
        let command = split.next().expect("No Command");
        let value = split
            .next()
            .expect("No Value")
            .parse::<i32>()
            .expect("Could not parse value");
        match command {
            "forward" => Command::Forward(value),
            "up" => Command::Up(value),
            "down" => Command::Down(value),
            _ => panic!("Not a valid command"),
        }
    }
}

struct SimpleState {
    forward: i32,
    depth: i32,
}
impl SimpleState {
    fn default() -> Self {
        SimpleState {
            forward: 0,
            depth: 0,
        }
    }
    fn control(self, cmd: &Command) -> Self {
        match cmd {
            Command::Forward(value) => SimpleState {
                forward: self.forward + value,
                ..self
            },
            Command::Up(value) => SimpleState {
                depth: self.depth - value,
                ..self
            },
            Command::Down(value) => SimpleState {
                depth: self.depth + value,
                ..self
            },
        }
    }
}

struct RealState {
    forward: i32,
    aim: i32,
    depth: i32,
}
impl RealState {
    fn default() -> Self {
        Self {
            forward: 0,
            aim: 0,
            depth: 0,
        }
    }
    fn control(self, cmd: &Command) -> Self {
        match cmd {
            Command::Forward(value) => Self {
                forward: self.forward + value,
                depth: self.depth + value * self.aim,
                ..self
            },
            Command::Up(value) => Self {
                aim: self.aim - value,
                ..self
            },
            Command::Down(value) => Self {
                aim: self.aim + value,
                ..self
            },
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let lines = BufReader::new(file).lines();

    // parse into command string
    let token_stream = lines
        .map(|l| Command::from_string(&l.expect("Can not read command")))
        .collect::<Vec<Command>>();

    // part1
    let simple_state = token_stream
        .iter()
        .fold(SimpleState::default(), |state, cmd| state.control(cmd));

    println!(
        "We are at position x: {} in depth: {}",
        simple_state.forward, simple_state.depth
    );
    println!(
        "Multiplied this is: {}",
        simple_state.forward * simple_state.depth
    );

    // part 2

    let real_state = token_stream
        .iter()
        .fold(RealState::default(), |state, cmd| state.control(cmd));
    println!(
        "But the real position is x: {} in depth: {}",
        real_state.forward, real_state.depth
    );
    println!(
        "Multiplied this is: {}",
        real_state.forward * real_state.depth
    );
}
