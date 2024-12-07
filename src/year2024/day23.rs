pub struct Parsed {}

pub fn parse(input: &str) -> Parsed {
    Parsed {}
}

pub fn part1(input: &Parsed) -> u64 {
    0
}
pub fn part2(input: &Parsed) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 0)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, 0)
    }
}
