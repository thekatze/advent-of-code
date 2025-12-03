pub struct Parsed(u64, u64);

trait AoC2025Day03Ext {
    fn get_joltage<const NUM_BATTERIES: usize>(&self) -> u64;
}

impl AoC2025Day03Ext for &[u8] {
    fn get_joltage<const NUM_BATTERIES: usize>(&self) -> u64 {
        let mut next_start_index = 0;

        (0..NUM_BATTERIES).rev().fold(0, |joltage, digit| {
            let last_possible_start = self.len() - digit;

            let (highest_number_index, highest_number) = (next_start_index..last_possible_start)
                .fold((0, 0), |(highest_number_index, highest_number), i| {
                    if self[i] > highest_number {
                        (i, self[i])
                    } else {
                        (highest_number_index, highest_number)
                    }
                });

            next_start_index = highest_number_index + 1;

            10 * joltage + (highest_number - b'0') as u64
        })
    }
}

pub fn parse(input: &str) -> Parsed {
    let (part1, part2) = input
        .as_bytes()
        .split(|c| *c == b'\n')
        .filter_map(|line| {
            (!line.is_empty()).then(|| (line.get_joltage::<2>(), line.get_joltage::<12>()))
        })
        .reduce(|agg, next| (agg.0 + next.0, agg.1 + next.1))
        .expect("input should not be empty");

    Parsed(part1, part2)
}

pub fn part1(input: &Parsed) -> u64 {
    input.0
}

pub fn part2(input: &Parsed) -> u64 {
    input.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 357)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, 3121910778619)
    }
}
