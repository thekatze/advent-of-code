pub struct Parsed<'a>(&'a [u8]);

pub fn parse(input: &str) -> Parsed<'_> {
    let bytes = input.as_bytes();

    // remove final newline
    // when we split by \n the final newline will yield an empty slice
    Parsed(&bytes[..bytes.len() - 1])
}

pub fn part1(input: &Parsed) -> u64 {
    input
        .0
        .split(|b| *b == b'\n')
        .map(|line| {
            let left_digit: u64 = line
                .iter()
                .find_map(|c: &u8| c.is_ascii_digit().then(|| u64::from(c - b'0')))
                .unwrap_or(0);
            let right_digit: u64 = line
                .iter()
                .rev()
                .find_map(|c: &u8| c.is_ascii_digit().then(|| u64::from(c - b'0')))
                .unwrap_or(0);

            left_digit * 10 + right_digit
        })
        .sum()
}

pub fn part2(input: &Parsed) -> u64 {
    let digits: [(&[u8], u64); _] = [
        (b"one", 1),
        (b"two", 2),
        (b"three", 3),
        (b"four", 4),
        (b"five", 5),
        (b"six", 6),
        (b"seven", 7),
        (b"eight", 8),
        (b"nine", 9),
    ];

    input
        .0
        .split(|b| *b == b'\n')
        .map(|line| {
            let mut at = 0;
            let left_digit = loop {
                let mut from_left = &line[at..];
                assert!(!from_left.is_empty());

                if let Some(actual_digit) = from_left
                    .first()
                    .and_then(|c: &u8| c.is_ascii_digit().then(|| u64::from(c - b'0')))
                {
                    break actual_digit;
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
                    .last()
                    .and_then(|c: &u8| c.is_ascii_digit().then(|| u64::from(c - b'0')))
                {
                    break actual_digit;
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
