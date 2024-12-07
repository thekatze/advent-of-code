use rayon::prelude::*;
use std::{fmt::format, iter::repeat_n};

use itertools::Itertools as _;

struct PartialEquation {
    result: u64,
    numbers: Vec<u64>,
}

enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl PartialEquation {
    fn has_valid_equation(&self, operators: &[Operation]) -> bool {
        let permutation_count = self.numbers.len() - 1;

        repeat_n(operators, permutation_count)
            .multi_cartesian_product() // -> all permutations with replacement
            .any(|operation_combination| {
                let equation_result = operation_combination
                    .iter()
                    .zip(self.numbers.iter().skip(1))
                    .fold(self.numbers[0], |acc, (op, num)| match op {
                        Operation::Add => acc + num,
                        Operation::Multiply => acc * num,
                        Operation::Concatenate => format!("{acc}{num}").parse().expect("number"),
                    });

                equation_result == self.result
            })
    }
}

pub struct Parsed(Vec<PartialEquation>);

pub fn parse(input: &str) -> Parsed {
    Parsed(
        input
            .lines()
            .map(|line| {
                let (result, numbers) = line
                    .split_once(": ")
                    .expect("invalid input: result must be separated by colon");

                let numbers = numbers
                    .split_whitespace()
                    .map(|number| number.parse())
                    .collect::<Result<Vec<_>, _>>()
                    .expect("invalid input: numbers");

                PartialEquation {
                    result: result.parse().expect("not a number"),
                    numbers,
                }
            })
            .collect(),
    )
}

pub fn part1(input: &Parsed) -> u64 {
    input
        .0
        .iter()
        .filter(|equation| equation.has_valid_equation(&[Operation::Add, Operation::Multiply]))
        .map(|equation| equation.result)
        .sum()
}

pub fn part2(input: &Parsed) -> u64 {
    input
        .0
        .par_iter()
        .filter(|equation| {
            equation.has_valid_equation(&[
                Operation::Add,
                Operation::Multiply,
                Operation::Concatenate,
            ])
        })
        .map(|equation| equation.result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 3749)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, 11387)
    }
}
