use crate::{puzzle::Parts, timer::format_duration};
use std::{fs::read_to_string, time::Instant};

const NAME: &str = "Day 6";
const INPUT_FILE: &str = "inputs/day_06.txt";

pub fn run(parts: Parts) {
    log::debug!("{}", NAME);

    let (problems_1, problems_2) = parse_input(
        &read_to_string(INPUT_FILE)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", INPUT_FILE, e)),
    )
    .expect("Failed to parse");

    let start = Instant::now();
    match parts {
        Parts::Both => {
            println!("{}", part_1(&problems_1));
            println!("{}", part_2(&problems_2));
        }
        Parts::First => {
            println!("{}", part_1(&problems_1));
        }
        Parts::Second => {
            println!("{}", part_2(&problems_2));
        }
    };
    let duration = start.elapsed();
    log::debug!("Duration {}", format_duration(duration));
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Product,
    Sum,
}

#[derive(Debug, Clone)]
struct Problem {
    nums: Vec<u64>,
    op: Op,
}

impl Problem {
    fn calculate(&self) -> u64 {
        match self.op {
            Op::Product => self.nums.iter().product(),
            Op::Sum => self.nums.iter().sum(),
        }
    }
}

fn parse_input(input: &str) -> Result<(Vec<Problem>, Vec<Problem>), anyhow::Error> {
    Ok((parse_1(input)?, parse_2(input)?))
}

// Parse input to problems for part one.
fn parse_1(input: &str) -> Result<Vec<Problem>, anyhow::Error> {
    let lines: Vec<&str> = input.lines().collect();

    // All lines except the last are number lines
    let num_lines: Vec<Vec<u64>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect();

    // Parse the operations from the last line
    let ops_line: Vec<Op> = lines
        .last()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| match s {
                    "*" => Some(Op::Product),
                    "+" => Some(Op::Sum),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default();

    // Build problems by iterating over columns
    let problems = (0..ops_line.len())
        .map(|i| Problem {
            nums: num_lines.iter().map(|num_line| num_line[i]).collect(),
            op: ops_line[i],
        })
        .collect();

    Ok(problems)
}

fn parse_2(input: &str) -> Result<Vec<Problem>, anyhow::Error> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut problems = Vec::new();
    let mut nums = Vec::new();

    for idx in (0..lines[0].len()).rev() {
        // Collect column characters by value to avoid double references
        let col: Vec<char> = lines
            .iter()
            .filter_map(|line| line.get(idx).copied())
            .collect();

        // Skip columns that are all spaces
        if col.iter().all(|&c| c == ' ') {
            continue;
        }

        // Parse number from column (reading bottom to top)
        let num = col
            .iter()
            .rev()
            .filter_map(|&c| c.to_digit(10))
            .enumerate()
            .fold(0, |acc, (place, d)| {
                acc + d as u64 * 10_u64.pow(place as u32)
            });

        nums.push(num);

        // Check for operator in last position and create problem if found
        if let Some(op) = match col.last() {
            Some(&'*') => Some(Op::Product),
            Some(&'+') => Some(Op::Sum),
            _ => None,
        } {
            problems.push(Problem {
                nums: std::mem::take(&mut nums),
                op,
            });
        }
    }

    Ok(problems)
}

fn part_1(problems: &[Problem]) -> u64 {
    solve(problems)
}

fn part_2(problems: &[Problem]) -> u64 {
    solve(problems)
}

fn solve(problems: &[Problem]) -> u64 {
    problems.iter().map(Problem::calculate).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part_1() {
        let (problems_1, _) = parse_input(TEST_INPUT).unwrap();
        let result = part_1(&problems_1);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part_2() {
        let (_, problems_2) = parse_input(TEST_INPUT).unwrap();
        let result = part_2(&problems_2);
        assert_eq!(result, 3263827);
    }
}
