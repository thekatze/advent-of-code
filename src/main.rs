#![allow(unused)]
#![allow(dead_code)]

use std::{
    collections::HashMap,
    fs,
    path::Path,
    time::{Duration, Instant},
};

use anyhow::Context;
use clap::{Parser, Subcommand};
use owo_colors::{OwoColorize as _, Style};

pub mod year2023;
pub mod year2024;
pub mod year2025;

macro_rules! solution {
    ($year:tt, $day:tt) => {{
        let run_fn = |data: String| {
            pastey::paste! {
                use [<year $year>]::[<day $day>]::*;
            }

            let parse_start = Instant::now();
            let input = parse(&data);
            let parse_end = Instant::now();

            let part1 = part1(&input);
            let part1_end = Instant::now();

            let part2 = part2(&input);
            let part2_end = Instant::now();

            RunResult {
                parse_time: parse_end - parse_start,
                part1: (part1_end - parse_end, part1),
                part2: (part2_end - part1_end, part2),
            }
        };

        (AoCDate($year, $day), Solution { run_fn })
    }};
}

fn get_solutions() -> HashMap<AoCDate, Solution> {
    HashMap::from_iter([
        // 2023
        solution!(2023, 01),
        // 2024
        solution!(2024, 01),
        solution!(2024, 02),
        solution!(2024, 03),
        solution!(2024, 04),
        solution!(2024, 05),
        solution!(2024, 06),
        solution!(2024, 07),
        solution!(2024, 08),
        // solution!(2024, 09) -- not finished,
        // 2025
        solution!(2025, 01),
    ])
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    Day { year: u16, day: u8 },
    Benchmark,
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct AoCDate(u16, u8);

fn load_day_files(
    path: &Path,
    year: u16,
    files: &mut HashMap<AoCDate, String>,
) -> anyhow::Result<()> {
    for entry in fs::read_dir(path).context("failed to read year folder")? {
        let entry = entry.context("failed to read entry")?;
        let p = entry.path();

        if p.is_file() {
            let day = p
                .file_stem()
                .and_then(|n| n.to_str())
                .and_then(|n| n.strip_prefix("day"))
                .and_then(|file_name| file_name.parse().ok())
                .context("could not extract day from path")?;

            let content = fs::read_to_string(&p)
                .with_context(|| format!("failed to read {}", p.display()))?;

            files.insert(AoCDate(year, day), content);
        }
    }
    Ok(())
}

fn load_all_input_files(base_path: &Path) -> anyhow::Result<HashMap<AoCDate, String>> {
    let mut files = HashMap::new();

    if let Ok(year_folders) = fs::read_dir(base_path) {
        for entry in year_folders {
            let year_path = entry.context("failed to read file")?.path();

            if year_path.is_dir() {
                let year = year_path
                    .file_stem()
                    .and_then(|file_name| file_name.to_str())
                    .and_then(|file_name| file_name.strip_prefix("year"))
                    .and_then(|number| number.parse().ok())
                    .context("could not extract year from path")?;

                load_day_files(&year_path, year, &mut files)?;
            }
        }
    }

    Ok(files)
}

struct RunResult {
    parse_time: Duration,
    part1: (Duration, u64),
    part2: (Duration, u64),
}

struct Solution {
    run_fn: fn(input: String) -> RunResult,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let input_files: HashMap<AoCDate, String> = match args.command {
        Command::Day { year, day } => HashMap::from_iter([(
            AoCDate(year, day),
            fs::read_to_string(format!("./input/year{}/day{:02}.txt", year, day))
                .context("could not find input file for day")?,
        )]),
        Command::Benchmark => load_all_input_files(Path::new("./input"))?,
    };

    let solutions = get_solutions();

    let mut results = input_files
        .into_iter()
        .filter_map(|(day, file)| {
            solutions
                .get(&day)
                .map(|solution| (day, (solution.run_fn)(file)))
        })
        .collect::<Vec<_>>();

    results.sort_unstable_by_key(|result| result.0);

    let mut total_time_elapsed = Duration::from_nanos(0);

    for (day, result) in results.iter() {
        println!(
            "{}",
            format!("=== Day {}-{} ===", day.0, day.1).magenta().bold()
        );
        println!(
            "{}",
            format!("    {}: {:#?}", "Parse".italic(), result.parse_time).bold()
        );
        println!(
            "{}",
            format!(
                "    {}: {} (in {:#?})",
                "Part 1".italic(),
                result.part1.1,
                result.part1.0
            )
            .bold()
        );
        println!(
            "{}",
            format!(
                "    {}: {} (in {:#?})",
                "Part 2".italic(),
                result.part2.1,
                result.part2.0
            )
            .bold()
        );

        total_time_elapsed += result.parse_time + result.part1.0 + result.part2.0;
    }

    let mut result_style = Style::new().bold();

    let expected_results = match args.command {
        Command::Day { .. } => 1,
        Command::Benchmark => solutions.len(),
    };

    if results.len() == expected_results {
        result_style = result_style.green();
    } else {
        result_style = result_style.red();
    }

    println!(
        "Ran {}/{} solutions in {:#?}",
        results.len().style(result_style),
        solutions.len().bold(),
        total_time_elapsed,
    );

    Ok(())
}
