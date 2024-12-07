use std::collections::BTreeMap;

pub struct Parsed(Vec<u64>, Vec<u64>);

pub fn parse(input: &str) -> Parsed {
    let (left, right) = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("   ").expect("invalid input format");
            (
                l.parse::<u64>().expect("input not a number"),
                r.parse::<u64>().expect("input not a number"),
            )
        })
        .unzip();

    Parsed(left, right)
}

pub fn part1(input: &Parsed) -> u64 {
    let (mut left, mut right) = (input.0.clone(), input.1.clone());
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}
pub fn part2(input: &Parsed) -> u64 {
    let (left, right) = (&input.0, &input.1);

    let mut occurence_count: BTreeMap<u64, u64> = BTreeMap::new();

    for number in right {
        let entry = occurence_count
            .entry(*number)
            .and_modify(|occurences| *occurences += 1)
            .or_insert(1);
    }

    left.iter()
        .map(|number| number * occurence_count.get(number).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 11)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, 31)
    }
}
