pub struct Parsed<'a>(&'a str);

pub fn parse(input: &str) -> Parsed<'_> {
    Parsed(input)
}

pub fn part1(input: &Parsed) -> u64 {
    input
        .0
        .lines()
        .map(|line| {
            let left_digit: u64 = line
                .chars()
                .find_map(|c: char| c.to_digit(10))
                .unwrap_or(0)
                .into();
            let right_digit: u64 = line
                .chars()
                .rev()
                .find_map(|c: char| c.to_digit(10))
                .unwrap_or(0)
                .into();

            left_digit * 10 + right_digit
        })
        .sum()
}

pub fn part2(input: &Parsed) -> u64 {
    let digits = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    input
        .0
        .lines()
        .map(|line| {
            let mut at = 0;
            let left_digit = loop {
                let mut from_left = &line[at..];
                assert!(!from_left.is_empty());

                if let Some(actual_digit) =
                    from_left.chars().next().map(|c| c.to_digit(10)).flatten()
                {
                    break actual_digit.into();
                }

                if let Some(text_digit) = digits
                    .iter()
                    .find_map(|d| from_left.starts_with(d.0).then_some(d.1))
                {
                    break text_digit;
                }

                at += 1;
            };

            let mut at = 0;
            let right_digit = loop {
                let mut from_right = &line[..line.len() - at];
                assert!(!from_right.is_empty());

                if let Some(actual_digit) = from_right
                    .chars()
                    .rev()
                    .next()
                    .map(|c| c.to_digit(10))
                    .flatten()
                {
                    break actual_digit.into();
                }

                if let Some(text_digit) = digits
                    .iter()
                    .find_map(|d| from_right.ends_with(d.0).then_some(d.1))
                {
                    break text_digit;
                }

                at += 1;
            };

            left_digit * 10 + right_digit
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const SAMPLE_INPUT_PART_2: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 142)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT_PART_2));
        assert_eq!(result, 281)
    }
}
