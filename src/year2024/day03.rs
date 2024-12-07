use regex::Regex;

pub struct Parsed(String);

pub fn parse(input: &str) -> Parsed {
    Parsed(input.to_owned()) // dont know what i need to parse yet, just pass the string
}

pub fn part1(input: &Parsed) -> u64 {
    let muls = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("invalid regex");
    muls.captures_iter(&input.0)
        .map(|capture| {
            let (_, params): (_, [_; 2]) = capture.extract();
            let lhs = params[0]
                .parse::<u64>()
                .expect("mul parameter should be number");
            let rhs = params[1]
                .parse::<u64>()
                .expect("mul parameter should be number");
            lhs * rhs
        })
        .sum()
}
pub fn part2(input: &Parsed) -> u64 {
    // capturing the opening and closing brackets of do and don't because i need to capture two
    // things in every case lol
    let actions =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").expect("invalid regex");

    let mut should_add = true;

    actions
        .captures_iter(&input.0)
        .map(|capture| {
            let substring = capture
                .get(0)
                .expect("get(0) is guaranteed to return a value")
                .as_str();

            if substring == "do()" {
                should_add = true;
                return 0;
            } else if substring == "don't()" {
                should_add = false;
                return 0;
            }

            if !should_add {
                return 0;
            }

            let (lhs, rhs) = (
                capture.get(1).expect("mul requires parameters").as_str(),
                capture.get(2).expect("mul requires parameters").as_str(),
            );

            let lhs = lhs.parse::<u64>().expect("mul parameter should be number");
            let rhs = rhs.parse::<u64>().expect("mul parameter should be number");

            lhs * rhs
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const SAMPLE_INPUT_2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 161)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT_2));
        assert_eq!(result, 48)
    }
}
