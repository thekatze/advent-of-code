#![allow(dead_code)]
#![allow(unused)]

use std::{
    env::args,
    fmt::Display,
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

use anyhow::Context;

pub mod year2024;

fn main() -> anyhow::Result<()> {
    let mut args = args();
    let (_program_name, year, day) = (
        args.next(),
        args.next().context("year not provided")?,
        args.next().context("day not provided")?,
    );

    let date = AoCDate(
        year.parse().context("year not a number")?,
        day.parse().context("day not a number")?,
    );

    let solution = solutions()
        .into_iter()
        .find(|solution| solution.date == date)
        .context("no solution for requested date")?;

    println!("{}", solution.run()?);

    Ok(())
}

macro_rules! solution {
    ($year:tt, $day:tt) => {{
        let year = stringify!($year);
        let day = stringify!($day);
        let path = std::path::Path::new("input")
            .join(year)
            .join(day)
            .with_extension("txt");

        let run_fn = |data: String| {
            use $year::$day::*;

            let parse_start = Instant::now();
            let input = parse(&data);
            let parse_end = Instant::now();

            let part1 = part1(&input);
            let part1_end = Instant::now();

            let part2 = part2(&input);
            let part2_end = Instant::now();

            Ok(RunResult {
                parse_time: parse_end - parse_start,
                part1: (part1_end - parse_end, part1),
                part2: (part2_end - part1_end, part2),
            })
        };

        Solution {
            date: (year, day)
                .try_into()
                .expect("module name format must be yearXXXX::dayXX"),
            path,
            run_fn,
        }
    }};
}

pub fn solutions() -> Vec<Solution> {
    vec![
        solution!(year2024, day01),
        solution!(year2024, day02),
        solution!(year2024, day03),
        solution!(year2024, day04),
        solution!(year2024, day05),
        solution!(year2024, day06),
        solution!(year2024, day07),
        solution!(year2024, day08),
        solution!(year2024, day09),
        solution!(year2024, day10),
        solution!(year2024, day11),
        solution!(year2024, day12),
        solution!(year2024, day13),
        solution!(year2024, day14),
        solution!(year2024, day15),
        solution!(year2024, day16),
        solution!(year2024, day17),
        solution!(year2024, day18),
        solution!(year2024, day19),
        solution!(year2024, day20),
        solution!(year2024, day21),
        solution!(year2024, day22),
        solution!(year2024, day23),
        solution!(year2024, day24),
        solution!(year2024, day25),
    ]
}

#[derive(PartialEq, Eq)]
pub struct AoCDate(u16, u8);

impl TryFrom<(&str, &str)> for AoCDate {
    type Error = ();

    fn try_from(value: (&str, &str)) -> std::result::Result<Self, Self::Error> {
        let (year, day) = value;

        let year = year[4..].parse::<u16>().map_err(|_| ())?;
        let day = day[3..].parse::<u8>().map_err(|_| ())?;

        Ok(AoCDate(year, day))
    }
}

pub struct Solution {
    date: AoCDate,
    path: PathBuf,
    run_fn: fn(String) -> anyhow::Result<RunResult>,
}

impl Solution {
    pub fn run(self) -> anyhow::Result<RunResult> {
        let input = fs::read_to_string(self.path)?;
        (self.run_fn)(input)
    }
}

pub struct RunResult {
    parse_time: Duration,
    part1: (Duration, String),
    part2: (Duration, String),
}

impl Display for RunResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Parse ({:?})", self.parse_time)?;
        writeln!(f, "Part 1 ({:?}): {}", self.part1.0, self.part1.1)?;
        writeln!(f, "Part 2 ({:?}): {}", self.part2.0, self.part2.1)?;

        Ok(())
    }
}
