#[allow(unused_imports)]
use day02_dive_common::{Command, Direction, SAMPLE_DATA, REAL_DATA};

fn main() {
    let result: u32 = do_work(REAL_DATA);
    println!("{}", result);
}

fn do_work<const N: usize>(data: [Command; N]) -> u32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in data {
        match command.direction {
            Direction::Forward => {
                position += command.distance;
                depth += command.distance * aim;
            },
            Direction::Down => aim += command.distance,
            Direction::Up => aim -= command.distance
        }
    }

    position * depth
}