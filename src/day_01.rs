use crate::{puzzle::Parts, timer::format_duration};
use std::time::Instant;

const NAME: &str = "Day 1";

pub fn run(parts: Parts) {
    log::debug!("{}", NAME);

    let input = String::from("");

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

fn part_1(input: &String) -> String {
    log::debug!("{}", input);
    return "".to_owned();
}

fn part_2(input: &String) -> String {
    log::debug!("{}", input);
    return "".to_owned();
}

#[cfg(test)]
mod test {}
