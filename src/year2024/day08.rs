use std::collections::BTreeMap;

use itertools::Itertools as _;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position(i64, i64);

pub struct Antennas(Vec<Position>);
pub struct Grid {
    width: i64,
    height: i64,
    antennas: BTreeMap<char, Antennas>,
}

pub fn parse(input: &str) -> Grid {
    let mut antennas: BTreeMap<char, Antennas> = BTreeMap::new();

    let characters_in_input = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, character)| (x, y, character))
        })
        .filter(|(_, _, character)| *character != '.');

    for (x, y, character) in characters_in_input {
        antennas
            .entry(character)
            .and_modify(|antennas| antennas.0.push(Position(x as i64, y as i64)))
            .or_insert_with(|| Antennas(vec![Position(x as i64, y as i64)]));
    }

    Grid {
        width: input.lines().next().expect("one line").len() as i64,
        height: input.lines().count() as i64,
        antennas,
    }
}

impl Antennas {
    fn get_antinodes_within(&self, size: (i64, i64)) -> impl Iterator<Item = Position> + use<'_> {
        self.0.iter().combinations(2).flat_map(move |antennas| {
            let a = antennas[0];
            let b = antennas[1];

            let distance = (b.0 - a.0, b.1 - a.1);

            [-1, 2]
                .iter()
                .map(move |offset| Position(a.0 + offset * distance.0, a.1 + offset * distance.1))
                .filter(move |pos| pos.0 >= 0 && pos.0 < size.0 && pos.1 >= 0 && pos.1 < size.1)
        })
    }

    fn get_part_2_antinodes_within(
        &self,
        size: (i64, i64),
    ) -> impl Iterator<Item = Position> + use<'_> {
        self.0.iter().combinations(2).flat_map(move |antennas| {
            let a = antennas[0];
            let b = antennas[1];

            let distance = (b.0 - a.0, b.1 - a.1);

            // its 7am and im tired. there must be a mathematical way to calculate
            // how many times the distance fits in the positive and negative direction.
            // but i just dont see it right now. sorry to disappoint
            (-100..100)
                .map(move |offset| Position(a.0 - offset * distance.0, a.1 - offset * distance.1))
                .filter(move |pos| pos.0 >= 0 && pos.0 < size.0 && pos.1 >= 0 && pos.1 < size.1)
        })
    }
}

pub fn part1(input: &Grid) -> u64 {
    input
        .antennas
        .iter()
        .flat_map(|(_, antennas)| antennas.get_antinodes_within((input.width, input.height)))
        .unique()
        .count() as u64
}
pub fn part2(input: &Grid) -> u64 {
    input
        .antennas
        .iter()
        .flat_map(|(_, antennas)| antennas.get_part_2_antinodes_within((input.width, input.height)))
        .unique()
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 14)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, 34)
    }
}
