use regex::Regex;
use std::fs;

#[derive(Default)]
struct Submarine {
    horizontal_position: i32,
    depth: i32,
}

impl Submarine {
    fn go(&mut self, direction: &MoveCommand) {
        match direction {
            MoveCommand::Forward(n) => self.horizontal_position += n,
            MoveCommand::Up(n) => self.depth -= n,
            MoveCommand::Down(n) => self.depth += n,
        }
    }

    fn multiply_position(&self) -> i32 {
        self.horizontal_position * self.depth
    }
}

#[derive(Default)]
struct AimingSubmarine {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}

impl AimingSubmarine {
    fn go(&mut self, direction: &MoveCommand) {
        match direction {
            MoveCommand::Forward(n) => {
                self.horizontal_position += n;
                self.depth += n * self.aim;
            }
            MoveCommand::Up(n) => self.aim -= n,
            MoveCommand::Down(n) => self.aim += n,
        }
    }

    fn multiply_position(&self) -> i32 {
        self.horizontal_position * self.depth
    }
}

enum MoveCommand {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl MoveCommand {
    fn parse_line(re: &Regex, line: &str) -> Option<Self> {
        // lazy as hell, using regex as an argument. but whatever!
        let caps = re.captures(line)?;

        let n = match caps[2].parse::<i32>() {
            Ok(n) => n,
            Err(_) => return None,
        };

        match caps[1].to_lowercase().as_str() {
            "forward" => Some(Self::Forward(n)),
            "up" => Some(Self::Up(n)),
            "down" => Some(Self::Down(n)),
            _ => None,
        }
    }
}

fn main() {
    let re = Regex::new(r"(\w+) (\d+)").expect("What int he lord");

    let input = fs::read_to_string("input.txt").expect("Uhh");
    let commands = input
        .lines()
        .filter_map(|l| MoveCommand::parse_line(&re, l));

    let mut submarine1 = Submarine::default();
    commands.clone().for_each(|c| submarine1.go(&c));

    println!(
        "submarine1 position multiplied: {}",
        submarine1.multiply_position()
    );

    let mut submarine2 = AimingSubmarine::default();
    commands.clone().for_each(|c| submarine2.go(&c));

    println!(
        "submarine2 position multiplied: {}",
        submarine2.multiply_position()
    );
}
