use crate::{puzzle::Parts, timer::format_duration};
use anyhow::anyhow;
use std::{any, error::Error, fs::read_to_string, time::Instant};

const NAME: &str = "Day 1";
const INPUT_FILE: &str = "inputs/day_01.txt";

pub fn run(parts: Parts) {
    log::debug!("{}", NAME);

    let input = read_to_string(INPUT_FILE)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", INPUT_FILE, e));

    let start = Instant::now();
    match parts {
        Parts::Both => {
            println!("{}", part_1(&input).unwrap());
            println!("{}", part_2(&input));
        }
        Parts::First => {
            println!("{}", part_1(&input).unwrap());
        }
        Parts::Second => {
            println!("{}", part_2(&input));
        }
    };
    let duration = start.elapsed();
    log::debug!("Duration {}", format_duration(duration));
}

fn part_1(input: &str) -> Result<i32, anyhow::Error> {
    log::debug!("{}", input);

    let turns = parse_input(input)?;

    let max_idx = 99;
    let min_idx = 0;

    let mut idx = 50;
    let mut n_zeros = 0;
    for turn in turns {
        log::debug!("idx before: {}", idx);
        log::debug!("turn: {:?}", turn);
        match turn.dir {
            Dir::L => {
                idx = if idx < turn.count {
                    log::debug!("Turning left and rolling over");
                    max_idx - turn.count - idx
                } else {
                    log::debug!("Turning left");
                    idx - turn.count
                }
            }
            Dir::R => {
                idx = if (max_idx - idx) < turn.count {
                    log::debug!("Turning right and rolling over");
                    max_idx + turn.count - idx - 1
                } else {
                    log::debug!("Turning right");
                    idx + turn.count
                }
            }
        }

        if idx == 0 {
            n_zeros += 1;
        }

        log::debug!("idx: {}", idx);
        log::debug!("n_zeros: {}\n", n_zeros);

        if idx < min_idx {
            panic!("Left rollover failed");
        }
        if idx > max_idx {
            panic!("Right rollover failed");
        }
    }

    Ok(n_zeros)
}

fn part_2(input: &str) -> &str {
    log::debug!("{}", input);
    ""
}

#[derive(Debug)]
enum Dir {
    L,
    R,
}

#[derive(Debug)]
struct Turn {
    dir: Dir,
    count: i32,
}

fn parse_input(input: &str) -> Result<Vec<Turn>, anyhow::Error> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Turn, anyhow::Error> {
    let (dir, count) = line.split_at(1);

    Ok(Turn {
        dir: match dir {
            "L" => Dir::L,
            "R" => Dir::R,
            _ => return Err(anyhow!("Invalid direction")),
        },
        count: count.parse()?,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part_1() {
        let result = part_1(&TEST_INPUT).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(&TEST_INPUT);
        assert_eq!(result, "");
    }
}
