#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

fn try_parse_rotation(value: &str) -> Result<i32, ()> {
    let (direction, amount) = value.split_at(1);
    let amount: i32 = amount.parse().map_err(|_| ())?;

    match direction {
        "L" => Ok(-amount),
        "R" => Ok(amount),
        _ => Err(()),
    }
}

pub struct Parsed(Vec<i32>);

pub fn parse(input: &str) -> Parsed {
    Parsed(
        input
            .lines()
            .map(|line| try_parse_rotation(line).expect("line not a rotation"))
            .collect(),
    )
}

pub fn part1(input: &Parsed) -> u64 {
    let mut dial = 50;

    input
        .0
        .iter()
        .filter(|rotation| {
            dial += *rotation;
            dial % 100 == 0
        })
        .count() as u64
}

pub fn part2(input: &Parsed) -> u64 {
    let mut dial = 50;

    input
        .0
        .iter()
        .map(|rotation| {
            let old_dial = dial;
            let old_dial_f32 = dial as f32;
            dial += *rotation;

            let dial_f32 = dial as f32;

            let mut clicks = ((old_dial_f32 / 100.0).floor() - ((dial_f32 / 100.0).floor())).abs();

            // do click if we land on 0 from turning left
            if rotation.is_negative() && dial % 100 == 0 {
                clicks += 1.0;
            }

            // don't click if we start from 0 and turn left
            if rotation.is_negative() && old_dial % 100 == 0 {
                clicks -= 1.0;
            }

            clicks as u64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 3)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, 6)
    }

    #[test]
    fn counts_clicks_correctly() {
        // Clicked 1 times rotating Left(24) from 24 to 0
        // Clicked 1 times rotating Left(122) from 0 to 78

        assert_eq!(
            super::part2(&parse(
                r"L26
L24
L122",
            )),
            2
        );

        // Clicked 2 times rotating Left(150) from 50 to 0
        assert_eq!(super::part2(&parse(r"L150",)), 2);

        // Clicked 1 time rotating Right(50) from 50 to 0
        assert_eq!(super::part2(&parse(r"R50",)), 1);

        // Clicked 1 time rotating Left(100) from 50 to 50
        assert_eq!(super::part2(&parse(r"L50",)), 1);
    }
}
