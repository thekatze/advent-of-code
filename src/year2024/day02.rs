use std::ops::Deref as _;

#[derive(Clone)]
struct Report(Vec<u32>);

impl Report {
    fn is_safe(&self) -> bool {
        let mut descending: Option<bool> = None;
        self.0.windows(2).all(|window| {
            let (left, right) = (window[0], window[1]);
            let window_descends = left > right;
            if (*descending.get_or_insert(window_descends) != window_descends) {
                return false;
            }

            let difference = left.abs_diff(right);

            (difference > 0 && difference < 4)
        })
    }
}

pub struct Parsed(Vec<Report>);

pub fn parse(input: &str) -> Parsed {
    Parsed(
        input
            .lines()
            .map(|line| {
                Report(
                    line.split_whitespace()
                        .map(str::parse::<u32>)
                        .collect::<Result<_, _>>()
                        .expect("Input not a number"),
                )
            })
            .collect(),
    )
}

pub fn part1(input: &Parsed) -> String {
    input
        .0
        .iter()
        .filter(|report| report.is_safe())
        .count()
        .to_string()
}
pub fn part2(input: &Parsed) -> String {
    input
        .0
        .iter()
        .filter(|report| {
            for i in 0..report.0.len() {
                let mut dampened_report = (*report).clone();
                dampened_report.0.remove(i);

                if (dampened_report.is_safe()) {
                    return true;
                }
            }

            false
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, "2")
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, "4")
    }
}
