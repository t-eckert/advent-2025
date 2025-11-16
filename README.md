# Advent of Code 2025

This is my repository for [Advent of Code](https://adventofcode.com/) for 2025. These are my solutions to the puzzles.

I am working in Rust using only the standard library. I'm not worried about submitting quickly, just enjoying the process of puzzle solving with my merely mortal mind. 

As a rule I will not be using any large language models or relevant AI such as Claude Code or GitHub Copilot both for writing code and for doing research. They're great, but they obviate the fundamental intention of this project. I'd rather have my own dumb solution than a smart one thought up for me. (Keep this in mind please if any of my code sucks)

## What is Advent of Code?

Advent of Code is a daily set of puzzle challenges which start on December 1st of each year. Each puzzle has a specific desired output as its solution. Each day has a single puzzle with two parts. Usually part two challenges some assumption you have made about the first part. 

## How this repository is set up

I have this repo set up like a simple CLI tool. I can use this to pull down the puzzle data and run the puzzle for each day. 

```
aoc pull 1 // pulls the puzzle input for day 1 and puts it in the git-ignored `inputs` dir
```

As I work on the puzzles, I will run tests with `cargo test day_01`. When I think I have the solution, I can use the run subcommand to run the full solution.

```
aoc run 1 // runs the puzzle for day 1
```
