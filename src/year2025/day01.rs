#[derive(Debug)]
enum Rotation {
    Left(u16),
    Right(u16),
}

impl Rotation {
    fn wrapping_rotate(&self, mut dial: &mut i32) -> u64 {
        let amount: i32 = match self {
            Rotation::Left(amount) => -(*amount as i32),
            Rotation::Right(amount) => *amount as i32,
        };

        *dial += amount;

        match dial {
            (..0) => {
                // TODO: solve arithmically
                let mut wraps = 0;
                while *dial < 0 {
                    wraps += 1;
                    *dial += 100;
                }
                wraps
            }
            (100..) => {
                // TODO: solve arithmically
                let mut wraps = 0;
                while *dial >= 100 {
                    wraps += 1;
                    *dial -= 100;
                }
                wraps
            }
            _ => 0,
        }
    }
}

impl TryFrom<&str> for Rotation {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (direction, amount) = value.split_at(1);
        let amount = amount.parse().map_err(|_| ())?;

        match direction {
            "L" => Ok(Rotation::Left(amount)),
            "R" => Ok(Rotation::Right(amount)),
            _ => Err(()),
        }
    }
}

pub struct Parsed(Vec<Rotation>);

pub fn parse(input: &str) -> Parsed {
    Parsed(
        input
            .lines()
            .map(|line| Rotation::try_from(line).expect("line not a rotation"))
            .collect(),
    )
}

pub fn part1(input: &Parsed) -> u64 {
    let mut dial = 50;

    input
        .0
        .iter()
        .filter(|rotation| {
            rotation.wrapping_rotate(&mut dial);
            dial == 0
        })
        .count() as u64
}

pub fn part2(input: &Parsed) -> u64 {
    0
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
        assert_eq!(result, 0)
    }
}
