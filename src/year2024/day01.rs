use std::collections::BTreeMap;

pub struct Parsed(Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Parsed {
    let numbers = input.lines().map(|line| {
        let (l, r) = line.split_once("   ").expect("invalid input format");
        (
            l.parse::<u32>().expect("input not a number"),
            r.parse::<u32>().expect("input not a number"),
        )
    });

    let mut left = Vec::new();
    let mut right = Vec::new();

    for (l, r) in numbers {
        left.push(l);
        right.push(r);
    }

    Parsed(left, right)
}

pub fn part1(input: &Parsed) -> String {
    let (mut left, mut right) = (input.0.clone(), input.1.clone());
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum::<u32>()
        .to_string()
}
pub fn part2(input: &Parsed) -> String {
    let (left, right) = (&input.0, &input.1);

    let mut occurence_count: BTreeMap<u32, u32> = BTreeMap::new();

    for number in right {
        let entry = occurence_count
            .entry(*number)
            .and_modify(|occurences| *occurences += 1)
            .or_insert(1);
    }

    left.iter()
        .map(|number| number * occurence_count.get(number).unwrap_or(&0))
        .sum::<u32>()
        .to_string()
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
        assert_eq!(result, "11")
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, "31")
    }
}
