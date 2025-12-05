use crate::{puzzle::Parts, timer::format_duration};
use std::{fs::read_to_string, time::Instant};

const NAME: &str = "Day 4";
const INPUT_FILE: &str = "inputs/day_04.txt";

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

// Grid represents the floor of the factory as viewed from overhead.
#[derive(Clone)]
struct Grid(Vec<Vec<bool>>);

impl Grid {
    fn neighbors(&self, x: usize, y: usize) -> Vec<bool> {
        let rows = self.0.len() as isize;
        let cols = self.0[0].len() as isize;
        let x = x as isize;
        let y = y as isize;

        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .filter_map(|(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && nx < cols && ny >= 0 && ny < rows {
                Some(self.0[ny as usize][nx as usize])
            } else {
                None
            }
        })
        .collect()
    }

    fn remove_roll(&mut self, x: usize, y: usize) {
        self.0[y][x] = false;
    }
}

impl FromIterator<Vec<bool>> for Grid {
    fn from_iter<T: IntoIterator<Item = Vec<bool>>>(iter: T) -> Self {
        Grid(iter.into_iter().collect())
    }
}

fn parse_input(input: &str) -> Result<Grid, anyhow::Error> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Vec<bool>, anyhow::Error> {
    Ok(line.chars().map(|c| c == '@').collect())
}

fn part_1(grid: &Grid) -> u64 {
    let mut count = 0;
    for (y, row) in grid.0.iter().enumerate() {
        for (x, is_roll) in row.iter().enumerate() {
            if *is_roll && grid.neighbors(x, y).iter().filter(|&&b| b).count() < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part_2(grid: &Grid) -> u64 {
    let mut grid = grid.clone();

    let mut count = 0;

    loop {
        let mut to_remove: Vec<(usize, usize)> = vec![];
        for (y, row) in grid.0.iter().enumerate() {
            for (x, is_roll) in row.iter().enumerate() {
                if *is_roll && grid.neighbors(x, y).iter().filter(|&&b| b).count() < 4 {
                    count += 1;
                    to_remove.push((x, y));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (x, y) in to_remove {
            grid.remove_roll(x, y);
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_1() {
        let rolls = parse_input(TEST_INPUT).unwrap();
        let result = part_1(&rolls);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_2() {
        let rolls = parse_input(TEST_INPUT).unwrap();
        let result = part_2(&rolls);
        assert_eq!(result, 43);
    }
}
