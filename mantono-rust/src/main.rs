#[macro_use]
extern crate clap;
extern crate lazy_static;

use aoc_macro::aoc;
use cfg::Puzzle;
use cfg::{Config, Part};
use input::get_input;
use logger::setup_logging;
use output::msg;
use output::print_out;

mod args;
mod cfg;
mod comp;
mod days;
mod fmt;
mod input;
mod logger;
mod output;

fn main() -> Result<(), String> {
    let cfg: Config = Config::from_args(args::args());
    setup_logging(cfg.verbosity_level);
    msg(&cfg);
    let puzzle_input: String = get_input(&cfg.puzzle, &cfg.session_token)?;
    let start = std::time::Instant::now();
    let output: String = run_puzzle(&cfg.puzzle, puzzle_input);
    let end = std::time::Instant::now();
    let elapsed = end.duration_since(start);
    print_out(output, elapsed, &cfg);

    Ok(())
}

#[aoc(4, "First")]
fn test() {
    println!("test")
}

fn run_puzzle(puzzle: &Puzzle, input: String) -> String {
    match (puzzle.day, &puzzle.part) {
        (1, Part::One) => days::day1::first(input),
        (1, Part::Two) => days::day1::second(input),
        (2, Part::One) => days::day2::first(input),
        (2, Part::Two) => days::day2::second(input),
        (3, Part::One) => days::day3::first(input),
        (3, Part::Two) => days::day3::second(input),
        (4, Part::One) => days::day4::first(input),
        (4, Part::Two) => days::day4::second(input),
        (5, Part::One) => days::day5::first(input),
        (5, Part::Two) => days::day5::second(input),
        (6, Part::One) => days::day6::first(input),
        (6, Part::Two) => days::day6::second(input),
        (7, Part::One) => days::day7::first(input),
        (7, Part::Two) => days::day7::second(input),
        (8, Part::One) => days::day8::first(input),
        (8, Part::Two) => days::day8::second(input),
        (9, Part::One) => days::day9::first(input),
        (9, Part::Two) => days::day9::second(input),
        (10, Part::One) => days::day10::first(input),
        (10, Part::Two) => days::day10::second(input),
        (11, Part::One) => days::day11::first(input),
        (11, Part::Two) => days::day11::second(input),
        (12, Part::One) => days::day12::first(input),
        (12, Part::Two) => days::day12::second(input),
        (13, Part::One) => days::day13::first(input),
        (13, Part::Two) => days::day13::second(input),
        (14, Part::One) => days::day14::first(input),
        (14, Part::Two) => days::day14::second(input),
        (20, Part::One) => days::day20::first(input),
        (20, Part::Two) => days::day20::second(input),
        _ => panic!("Not supported"),
    }
}
