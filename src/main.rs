mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod puzzle;
mod timer;

use anyhow::anyhow;
use std::{env, fs};

use clap::{Parser, Subcommand};

use crate::puzzle::Parts;
use reqwest::blocking::Client;

#[derive(Parser)]
#[command(name = "aoc")]
#[command(about = "Advent of Code CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Set the logging level
    #[arg(long, global = true, value_enum)]
    log: Option<log::Level>,
}

#[derive(Subcommand)]
enum Commands {
    /// Pull the input for a specific day
    Pull {
        /// The day to pull (1-25)
        #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
        day: u8,
    },
    /// Run the solution for a specific day
    Run {
        /// The day to run (1-25)
        #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
        day: u8,

        /// Run only the first part
        #[arg(long, conflicts_with = "second")]
        first: bool,

        /// Run only the second part
        #[arg(long, conflicts_with = "first")]
        second: bool,
    },
}

fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv()?;

    let cli = Cli::parse();

    if let Some(level) = cli.log {
        simple_logger::init_with_level(level)?;
    }

    match cli.command {
        Commands::Pull { day } => pull(day),
        Commands::Run { day, first, second } => match (first, second) {
            (false, false) => run(day, Parts::Both),
            (true, false) => run(day, Parts::First),
            (false, true) => run(day, Parts::Second),
            (true, true) => unreachable!("prevented by Clap conflicts_with"),
        },
    }
}

fn pull(day: u8) -> Result<(), anyhow::Error> {
    let url = format!("https://adventofcode.com/2025/day/{}/input", day);

    let client = Client::new();

    let session = env::var("SESSION")?;

    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()?
        .error_for_status()?;

    fs::write(format!("./inputs/day_{:02}.txt", day), response.bytes()?)
        .map_err(|e| anyhow!(e.to_string()))
}

fn run(day: u8, parts: Parts) -> Result<(), anyhow::Error> {
    match day {
        1 => day_01::run(parts),
        2 => day_02::run(parts),
        3 => day_03::run(parts),
        4 => day_04::run(parts),
        5 => day_05::run(parts),
        6..=12 => return Err(anyhow!("I haven't implemented these yet.")),
        _ => return Err(anyhow!("Invalid day. Valid days are 1-12 inclusive.")),
    };

    return Ok(());
}
