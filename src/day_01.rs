use crate::{puzzle::Parts, timer::format_duration};
use std::{fs::read_to_string, time::Instant};

const NAME: &str = "Day 1";
const INPUT_FILE: &str = "inputs/day_01.txt";

pub fn run(parts: Parts) {
    log::debug!("{}", NAME);

    let input = read_to_string(INPUT_FILE)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", INPUT_FILE, e));

    let start = Instant::now();
    match parts {
        Parts::Both => {
            println!("{}", part_1(&input));
            println!("{}", part_2(&input));
        }
        Parts::First => {
            println!("{}", part_1(&input));
        }
        Parts::Second => {
            println!("{}", part_2(&input));
        }
    };
    let duration = start.elapsed();
    log::debug!("Duration {}", format_duration(duration));
}

fn part_1(input: &str) -> &str {
    log::debug!("{}", input);
    ""
}

fn part_2(input: &str) -> &str {
    log::debug!("{}", input);
    ""
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part_1() {
        let result = part_1(&TEST_INPUT);
        assert_eq!(result, "");
    }

    #[test]
    fn test_part_2() {
        let result = part_2(&TEST_INPUT);
        assert_eq!(result, "");
    }
}
