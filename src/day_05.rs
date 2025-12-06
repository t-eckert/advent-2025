use crate::{puzzle::Parts, timer::format_duration};
use regex::Regex;
use std::{cmp::max, fs::read_to_string, time::Instant};

const NAME: &str = "Day 5";
const INPUT_FILE: &str = "inputs/day_05.txt";

pub fn run(parts: Parts) {
    log::debug!("{}", NAME);

    let grid = parse_input(
        read_to_string(INPUT_FILE)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", INPUT_FILE, e))
            .trim(),
    )
    .expect("Failed to parse");

    let start = Instant::now();
    match parts {
        Parts::Both => {
            println!("{}", part_1(&grid));
            println!("{}", part_2(&grid));
        }
        Parts::First => {
            println!("{}", part_1(&grid));
        }
        Parts::Second => {
            println!("{}", part_2(&grid));
        }
    };
    let duration = start.elapsed();
    log::debug!("Duration {}", format_duration(duration));
}

// Ranges of fresh ingredients. Ranges are inclusive.
#[derive(Debug, Clone)]
struct Range {
    from: u64,
    to: u64,
}

impl Range {
    fn contains(&self, n: &u64) -> bool {
        &self.from <= n && n <= &self.to
    }
}

#[derive(Debug, Clone)]
struct DB {
    pub ranges: Vec<Range>,
    pub ids: Vec<u64>,
}

fn parse_input(input: &str) -> Result<DB, anyhow::Error> {
    let re_range = Regex::new(r"(\d+)-(\d+)")?;

    let mut ranges: Vec<Range> = vec![];
    let mut ids: Vec<u64> = vec![];
    for line in input.lines() {
        // Parse in ranges
        if let Some(captures) = re_range.captures(line) {
            ranges.push(Range {
                from: captures[1].parse()?,
                to: captures[2].parse()?,
            })
        } else if line.is_empty() {
            // Skip the blank line.
            continue;
        } else {
            ids.push(line.parse()?);
        }
    }

    Ok(DB { ranges, ids })
}

fn part_1(db: &DB) -> u64 {
    let mut n_fresh = 0;

    for id in db.ids.iter() {
        if db.ranges.iter().any(|range| range.contains(id)) {
            n_fresh += 1;
        }
    }

    n_fresh
}

fn part_2(db: &DB) -> u64 {
    let mut db = db.clone();

    // Order the ranges by their `from` value.
    db.ranges.sort_by(|r1, r2| r1.from.cmp(&r2.from));

    let mut n_fresh = 0;
    let mut bound = 0;
    for range in db.ranges {
        if range.to < bound {
            continue;
        }

        n_fresh += range.to - max(bound, range.from) + 1;

        bound = max(bound, range.to + 1);
    }

    n_fresh
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_1() {
        let db = parse_input(TEST_INPUT).unwrap();
        let result = part_1(&db);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let db = parse_input(TEST_INPUT).unwrap();
        let result = part_2(&db);
        assert_eq!(result, 14);
    }
}
