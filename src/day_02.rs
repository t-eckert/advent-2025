use crate::{puzzle::Parts, timer::format_duration};
use anyhow::anyhow;
use std::{fs::read_to_string, time::Instant};

const NAME: &str = "Day 2";
const INPUT_FILE: &str = "inputs/day_02.txt";

pub fn run(parts: Parts) {
    log::debug!("{}", NAME);

    let range = parse_input(
        read_to_string(INPUT_FILE)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", INPUT_FILE, e))
            .trim(),
    )
    .expect("Failed to parse");

    let start = Instant::now();
    match parts {
        Parts::Both => {
            println!("{}", part_1(&range));
            println!("{}", part_2(&range));
        }
        Parts::First => {
            println!("{}", part_1(&range));
        }
        Parts::Second => {
            println!("{}", part_2(&range));
        }
    };
    let duration = start.elapsed();
    log::debug!("Duration {}", format_duration(duration));
}

#[derive(Debug)]
struct Range {
    from: u64,
    to: u64,
}

fn parse_input(input: &str) -> Result<Vec<Range>, anyhow::Error> {
    input.split(',').map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Range, anyhow::Error> {
    let (from, to) = line
        .split_once('-')
        .ok_or(anyhow!("Missing dash in range"))?;

    Ok(Range {
        from: from.parse()?,
        to: to.parse()?,
    })
}

fn part_1(ranges: &[Range]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| range.from..=range.to)
        .filter(|&id| is_invalid_part_1(id))
        .sum()
}

fn part_2(ranges: &[Range]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| range.from..=range.to)
        .filter(|&id| is_invalid_part_2(id))
        .sum()
}

fn has_even_n_digits(num: u64) -> bool {
    num.to_string().len().is_multiple_of(2)
}

// In part one, an ID is invalid if it is composed of two repeated digit sequences.
fn is_invalid_part_1(id: u64) -> bool {
    // the invalid IDs can only include numbers with an even number of digits.
    if !has_even_n_digits(id) {
        return false;
    }

    let s = id.to_string();
    let (left, right) = s.split_at(s.len() / 2);

    left == right
}

// In part two, an ID is invalid if it is composed exclusively of any repeated digit sequences.
fn is_invalid_part_2(id: u64) -> bool {
    let id_string = id.to_string();
    let len = id_string.len();
    let bytes = id_string.as_bytes();

    // We check the set of sequence lengths using:
    //   (1..=len / 2).filter(|&seq_len| len.is_multiple_of(seq_len))
    // Which includes all sequence lengths 1 to half the length of the ID (inclusive),
    // filtering out all sequence lengths that are not a factor of the length of the ID.
    // In other words, the sequence must fit into the ID a "whole" number of times.
    for seq_len in (1..=len / 2).filter(|&seq_len| len.is_multiple_of(seq_len)) {
        // The first chunk...
        let first = &bytes[0..seq_len];
        // ...must be equal to all of the other chunks.
        if bytes.chunks(seq_len).all(|seq| seq == first) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1() {
        let ranges = parse_input(TEST_INPUT).unwrap();
        let result = part_1(&ranges);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part_2() {
        let ranges = parse_input(TEST_INPUT).unwrap();
        let result = part_2(&ranges);
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn test_has_even_digits() {
        // Even digits
        assert!(has_even_n_digits(20));
        assert!(has_even_n_digits(4242));

        // Odd digits
        assert!(!has_even_n_digits(0));
        assert!(!has_even_n_digits(5));
        assert!(!has_even_n_digits(100));
        assert!(!has_even_n_digits(3429504));
    }

    #[test]
    fn test_is_invalid_part_2() {
        // Should be valid
        assert!(!is_invalid_part_2(1234123));

        // Should be invalid
        assert!(is_invalid_part_2(424242));
    }
}
