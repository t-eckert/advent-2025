use crate::{puzzle::Parts, timer::format_duration};
use std::{fs::read_to_string, time::Instant};

const NAME: &str = "Day 3";
const INPUT_FILE: &str = "inputs/day_03.txt";

pub fn run(parts: Parts) {
    log::debug!("{}", NAME);

    let banks = parse_input(
        read_to_string(INPUT_FILE)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", INPUT_FILE, e))
            .trim(),
    )
    .expect("Failed to parse");

    let start = Instant::now();
    match parts {
        Parts::Both => {
            println!("{}", part_1(&banks));
            println!("{}", part_2(&banks));
        }
        Parts::First => {
            println!("{}", part_1(&banks));
        }
        Parts::Second => {
            println!("{}", part_2(&banks));
        }
    };
    let duration = start.elapsed();
    log::debug!("Duration {}", format_duration(duration));
}
fn parse_input(input: &str) -> Result<Vec<Vec<u32>>, anyhow::Error> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Vec<u32>, anyhow::Error> {
    Ok(line.chars().map(|d| d.to_digit(10).unwrap()).collect())
}

fn part_1(banks: &[Vec<u32>]) -> u64 {
    banks
        .iter()
        .map(|bank| calculate_max_joltage(bank, 2))
        .sum()
}

fn part_2(banks: &[Vec<u32>]) -> u64 {
    banks
        .iter()
        .map(|bank| calculate_max_joltage(bank, 12))
        .sum()
}

fn calculate_max_joltage(bank: &[u32], batteries: usize) -> u64 {
    let mut index = 0;
    let mut max = 0;

    for place in (0..batteries).rev() {
        let (idx, m) = max_with_index(&bank[index..bank.len() - place]).unwrap();
        index += idx + 1;
        max += m as u64 * 10_u64.pow(place as u32);
    }

    max
}

fn max_with_index(nums: &[u32]) -> Option<(usize, u32)> {
    nums.iter()
        .enumerate()
        .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i2.cmp(i1)))
        .map(|(index, value)| (index, *value))
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_1() {
        let banks = parse_input(TEST_INPUT).unwrap();
        let result = part_1(&banks);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part_2() {
        let banks = parse_input(TEST_INPUT).unwrap();
        let result = part_2(&banks);
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn test_calculate_max_joltage() {
        let banks = parse_input(TEST_INPUT).unwrap();
        let max_js: Vec<u64> = vec![987654321111, 811111111119, 434234234278, 888911112111];

        for (bank, max_joltage) in banks.iter().zip(max_js) {
            assert_eq!(
                max_joltage,
                calculate_max_joltage(bank, max_joltage.to_string().len())
            );
        }
    }
}
