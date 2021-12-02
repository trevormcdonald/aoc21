use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

struct Instruction {
    direction: Direction,
    magnitude: i32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        let input: Vec<&str> = input.split_whitespace().collect();
        if input.len() != 2 {
            return Err(());
        }
        return Ok(Instruction {
            direction: Direction::from_str(input.get(0).unwrap()).unwrap(),
            magnitude: input.get(1).unwrap().parse::<i32>().unwrap(),
        });
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let instructions: Vec<Instruction> = contents
        .lines()
        .map(|x| Instruction::from_str(x).unwrap())
        .collect();

    let answer_one = instructions
        .iter()
        .fold((0, 0), |acc, x| match x.direction {
            Direction::Forward => (acc.0 + x.magnitude, acc.1),
            Direction::Down => (acc.0, acc.1 + x.magnitude),
            Direction::Up => (acc.0, acc.1 - x.magnitude),
        });
    println!("One: {:?}", answer_one.0 * answer_one.1);

    let answer_two = instructions
        .iter()
        .fold((0, 0, 0), |acc, x| match x.direction {
            Direction::Forward => (acc.0 + x.magnitude, acc.1 + acc.2 * x.magnitude, acc.2),
            Direction::Down => (acc.0, acc.1, acc.2 + x.magnitude),
            Direction::Up => (acc.0, acc.1, acc.2 - x.magnitude),
        });
    println!("Two: {:?}", answer_two.0 * answer_two.1);
}
