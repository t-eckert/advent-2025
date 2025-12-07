use crate::{puzzle::Parts, timer::format_duration};
use std::{fs::read_to_string, time::Instant};

const NAME: &str = "Day 11";
const INPUT_FILE: &str = "inputs/day_11.txt";

pub fn run(parts: Parts) {
    log::debug!("{}", NAME);

    let parsed_input = parse_input(
        &read_to_string(INPUT_FILE)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", INPUT_FILE, e)),
    )
    .expect("Failed to parse");

    let start = Instant::now();
    match parts {
        Parts::Both => {
            println!("{}", part_1(&parsed_input));
            println!("{}", part_2(&parsed_input));
        }
        Parts::First => {
            println!("{}", part_1(&parsed_input));
        }
        Parts::Second => {
            println!("{}", part_2(&parsed_input));
        }
    };
    let duration = start.elapsed();
    log::debug!("Duration {}", format_duration(duration));
}

fn parse_input(input: &str) -> Result<Vec<u32>, anyhow::Error> {
    Ok(vec![])
}

fn part_1(problems: &[u32]) -> u64 {
    0
}

fn part_2(problems: &[u32]) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part_1() {
        let parsed_input = parse_input(TEST_INPUT).unwrap();
        let result = part_1(&parsed_input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_2() {
        let parsed_input = parse_input(TEST_INPUT).unwrap();
        let result = part_2(&parsed_input);
        assert_eq!(result, 0);
    }
}
