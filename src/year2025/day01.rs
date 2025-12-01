#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

fn try_parse_rotation(value: &[u8]) -> Result<i32, ()> {
    let (direction, amount) = value.split_first_chunk::<1>().ok_or(())?;
    let amount: i32 = atoi::atoi(amount).ok_or(())?;

    match direction {
        [b'L'] => Ok(-amount),
        [b'R'] => Ok(amount),
        _ => Err(()),
    }
}

pub struct Parsed(Vec<i32>);

pub fn parse(input: &str) -> Parsed {
    Parsed(
        input
            .as_bytes()
            .split(|c| *c == b'\n')
            .filter_map(|line| {
                (!line.is_empty()).then(|| try_parse_rotation(line).expect("line not a rotation"))
            })
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

#[inline(never)]
pub fn part2(input: &Parsed) -> u64 {
    let mut dial = 50;

    input
        .0
        .iter()
        .map(|rotation| {
            let previous_dial = dial;
            dial += *rotation;

            let mut clicks = ((previous_dial as f32 / 100.0).floor()
                - ((dial as f32 / 100.0).floor()))
            .abs() as u64;

            // NOTE: the compiler can optimize the following code to conditional moves.
            // the explicit form looks like this:
            //
            // let negative = rotation.is_negative() as i32;
            // let landed_on_zero = (dial % 100 == 0) as i32;
            // let started_on_zero = (previous_dial % 100 == 0) as i32;
            // return (clicks + negative * (landed_on_zero - started_on_zero)) as u64;

            // do click if we land on 0 from turning left
            if rotation.is_negative() && dial % 100 == 0 {
                clicks += 1;
            }

            // don't click if we start from 0 and turn left
            if rotation.is_negative() && previous_dial % 100 == 0 {
                clicks -= 1;
            }

            clicks
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
L122
",
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
