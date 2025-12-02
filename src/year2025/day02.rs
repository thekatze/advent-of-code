#[derive(Clone, Copy, Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl IntoIterator for Range {
    type Item = u64;

    type IntoIter = std::ops::RangeInclusive<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

pub struct Parsed(Vec<Range>);

pub fn parse(input: &str) -> Parsed {
    Parsed(
        input
            .as_bytes()
            .split(|c| *c == b',')
            .map(|range| {
                let mut parts = range.splitn(2, |c| *c == b'-');
                let (from, to) = (
                    parts.next().expect("range must be delimited by -"),
                    parts.next().expect("range must be delimited by -"),
                );

                Range {
                    start: atoi::atoi(from).expect("range must be a number"),
                    end: atoi::atoi(to).expect("range must be a number"),
                }
            })
            .collect(),
    )
}

pub fn part1(input: &Parsed) -> u64 {
    input
        .0
        .iter()
        .flat_map(|range| range.into_iter())
        .filter(|number| {
            let digits = number.ilog10() + 1;

            // uneven length can be skipped outright
            if (digits & 0b1) == 0b1 {
                return false;
            }

            let digit_halfing_power_of_ten = 10_u64.pow(digits / 2);
            let high = number / digit_halfing_power_of_ten;
            let low = number - high * digit_halfing_power_of_ten;

            high == low
        })
        .sum()
}

pub fn part2(input: &Parsed) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 1227775554)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, 0)
    }
}
