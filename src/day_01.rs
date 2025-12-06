use crate::{puzzle::Parts, timer::format_duration};
use anyhow::anyhow;
use std::{fs::read_to_string, time::Instant};

const NAME: &str = "Day 1";
const INPUT_FILE: &str = "inputs/day_01.txt";

pub fn run(parts: Parts) {
    log::debug!("{}", NAME);

    let turns = parse_input(
        &read_to_string(INPUT_FILE)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", INPUT_FILE, e)),
    )
    .expect("Failed to parse");

    let start = Instant::now();
    match parts {
        Parts::Both => {
            println!("{}", part_1(&turns));
            println!("{}", part_2(&turns));
        }
        Parts::First => {
            println!("{}", part_1(&turns));
        }
        Parts::Second => {
            println!("{}", part_2(&turns));
        }
    };
    let duration = start.elapsed();
    log::debug!("Duration {}", format_duration(duration));
}

fn parse_input(input: &str) -> Result<Vec<i32>, anyhow::Error> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<i32, anyhow::Error> {
    let (dir, count) = line.split_at(1);

    let sign = if dir == "L" {
        -1
    } else if dir == "R" {
        1
    } else {
        return Err(anyhow!("Invalid direction"));
    };

    Ok(sign * count.parse::<i32>()?)
}

fn part_1(turns: &Vec<i32>) -> i32 {
    let max_idx = 99;
    let min_idx = 0;
    let n_indices = 100; // Because we include the 0 as an option

    let mut idx = 50;
    let mut n_zeros = 0;
    for turn in turns {
        // Apply, then normalize
        idx += turn;

        // Handle negative rollover.
        while idx < min_idx {
            idx += n_indices;
        }

        // Handle positive rollover
        while idx > max_idx {
            idx -= n_indices;
        }

        if idx == 0 {
            n_zeros += 1;
        }
    }

    n_zeros
}

fn part_2(turns: &Vec<i32>) -> i32 {
    let max_idx = 99;
    let min_idx = 0;
    let n_indices = 100; // Because we include the 0 as an option

    let mut idx = 50;
    let mut n_zeros = 0;
    for turn in turns {
        // Apply, then normalize
        idx += turn;

        // Handle negative rollover.
        while idx < min_idx {
            n_zeros += 1;
            idx += n_indices;
        }

        // Handle positive rollover
        while idx > max_idx {
            n_zeros += 1;
            idx -= n_indices;
        }
    }

    n_zeros
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
        let turns = parse_input(TEST_INPUT).unwrap();
        let result = part_1(&turns);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let turns = parse_input(TEST_INPUT).unwrap();
        let result = part_2(&turns);
        assert_eq!(result, 6);
    }
}
