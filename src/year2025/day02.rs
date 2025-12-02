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

trait AoC2025Day02Ext {
    fn is_invalid_part_1(&self) -> bool;
    fn is_invalid_part_2(&self) -> bool;
}

impl AoC2025Day02Ext for u64 {
    fn is_invalid_part_1(&self) -> bool {
        let digits = self.ilog10() + 1;

        // uneven length can be skipped outright
        if (digits & 0b1) == 0b1 {
            return false;
        }

        let digit_halfing_power_of_ten = 10_u64.pow(digits / 2);
        let high = self / digit_halfing_power_of_ten;
        let low = self - high * digit_halfing_power_of_ten;

        high == low
    }

    fn is_invalid_part_2(&self) -> bool {
        let digits = self.ilog10() + 1;
        let half_digits = digits / 2;

        'next_digit_length: for d in 1..=half_digits {
            let (max_num_checks, remainder) = (digits / d, digits % d);
            // if the number is not evenly divisible by digit count
            // we can skip this number
            if remainder > 0 {
                continue;
            }

            let pwr = 10_u64.pow(digits - 1 * d);
            let needle = self / pwr;
            let mut haystack = self - needle * pwr;

            for check_n in (0..max_num_checks - 1).rev() {
                let pwr = 10_u64.pow(check_n * d);
                let compare = haystack / pwr;

                if (compare != needle) {
                    // refuted this pattern, try next digit length
                    continue 'next_digit_length;
                }

                haystack = haystack - compare * pwr;
            }

            // could not refute this pattern, must be repeating
            return true;
        }

        false
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
        .filter(|number| number.is_invalid_part_1())
        .sum()
}

pub fn part2(input: &Parsed) -> u64 {
    input
        .0
        .iter()
        .flat_map(|range| range.into_iter())
        .filter(|number| number.is_invalid_part_2())
        .sum()
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
        assert_eq!(result, 4174379265)
    }
}
