use crate::{puzzle::Parts, timer::format_duration};
use std::{fs::read_to_string, time::Instant};

const NAME: &str = "Day 7";
const INPUT_FILE: &str = "inputs/day_07.txt";

pub fn run(parts: Parts) {
    log::debug!("{}", NAME);

    let field = parse_input(
        &read_to_string(INPUT_FILE)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", INPUT_FILE, e)),
    )
    .expect("Failed to parse");

    let start = Instant::now();
    match parts {
        Parts::Both => {
            println!("{}", part_1(&field));
            println!("{}", part_2(&field));
        }
        Parts::First => {
            println!("{}", part_1(&field));
        }
        Parts::Second => {
            println!("{}", part_2(&field));
        }
    };
    let duration = start.elapsed();
    log::debug!("Duration {}", format_duration(duration));
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Entity {
    Source,
    Splitter,
    Beam,
}

#[derive(Debug, Clone)]
struct Field {
    rows: Vec<Vec<(usize, Entity)>>,
}

impl Field {
    fn place_beam(&mut self, row: usize, col: usize) {
        if self.rows.len() <= row {
            return;
        }

        self.rows[row].push((col, Entity::Beam));
    }
}

fn parse_input(input: &str) -> Result<Field, anyhow::Error> {
    let rows = input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| match c {
                    'S' => Some((i, Entity::Source)),
                    '^' => Some((i, Entity::Splitter)),
                    _ => None,
                })
                .collect()
        })
        .collect();

    Ok(Field { rows })
}

fn part_1(field: &Field) -> u64 {
    let mut field = field.clone();

    let mut n_splits = 0;
    let num_rows = field.rows.len();
    for row in 0..num_rows - 1 {
        let entities: Vec<_> = field.rows[row].clone();

        for (col, ent) in entities {
            dbg!(&col, &ent);
            match ent {
                Entity::Source => {
                    field.place_beam(row + 1, col);
                }
                Entity::Splitter => {
                    if row > 0 && field.rows[row - 1].contains(&(col, Entity::Beam)) {
                        n_splits += 1;

                        field.place_beam(row, col - 1);
                        if row < field.rows.len()
                            && !field.rows[row + 1].contains(&(col, Entity::Splitter))
                        {
                            field.place_beam(row + 1, col - 1);
                        }

                        field.place_beam(row, col + 1);
                        if row < field.rows.len()
                            && !field.rows[row + 1].contains(&(col, Entity::Splitter))
                        {
                            field.place_beam(row + 1, col + 1);
                        }
                    }
                }
                Entity::Beam => {
                    if row < field.rows.len()
                        && !field.rows[row + 1].contains(&(col, Entity::Splitter))
                    {
                        field.place_beam(row + 1, col);
                    }
                }
            }
        }
    }

    dbg!(field);

    n_splits
}

fn part_2(field: &Field) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part_1() {
        let field = parse_input(TEST_INPUT).unwrap();
        let result = part_1(&field);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_2() {
        let field = parse_input(TEST_INPUT).unwrap();
        let result = part_2(&field);
        assert_eq!(result, 0);
    }
}
