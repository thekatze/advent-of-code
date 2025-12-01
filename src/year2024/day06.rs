use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Heading {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Visited,
    Obstructed,
}

#[derive(Clone, Debug)]
pub struct Parsed {
    tiles: Vec<Tile>,
    character_position: usize,
    heading: Heading,
    width: usize,
    height: usize,
}

impl Parsed {
    fn get_next_position(&self) -> Option<usize> {
        let x = self.character_position % self.width;
        let y = self.character_position / self.width;
        match self.heading {
            Heading::North => (y != 0).then(|| (y - 1) * self.width + x),
            Heading::East => (x != self.width - 1).then(|| y * self.width + x + 1),
            Heading::South => (y != self.height - 1).then(|| (y + 1) * self.width + x),
            Heading::West => (x != 0).then(|| y * self.width + x - 1),
        }
    }

    fn walk(&mut self) -> bool {
        let mut visited: HashSet<(usize, Heading)> = HashSet::new();
        while let Some(next) = self.get_next_position() {
            if self.tiles[next] == Tile::Obstructed {
                self.heading = match self.heading {
                    Heading::North => Heading::East,
                    Heading::East => Heading::South,
                    Heading::South => Heading::West,
                    Heading::West => Heading::North,
                };

                continue;
            }

            if (!visited.insert((next, self.heading.clone()))) {
                // cycle detected
                return false;
            }

            self.tiles[next] = Tile::Visited;
            self.character_position = next;
        }

        true
    }
}

pub fn parse(input: &str) -> Parsed {
    let height = input.lines().count();
    let width = input.lines().next().expect("min one line").len();

    let mut current_index = 0;
    let mut character_position: Option<_> = None;

    let tiles = input
        .chars()
        .filter_map(|input| {
            let result = match input {
                '.' => Some(Tile::Empty),
                '#' => Some(Tile::Obstructed),
                '^' => {
                    character_position = Some(current_index);
                    Some(Tile::Visited)
                }
                _ => return None, // skip index increment
            };

            current_index += 1;

            result
        })
        .collect();

    Parsed {
        tiles,
        character_position: character_position.expect("map must have starting pos"),
        heading: Heading::North,
        width,
        height,
    }
}

pub fn part1(input: &Parsed) -> u64 {
    let mut map = input.clone();
    map.walk();

    map.tiles
        .iter()
        .filter(|tile| **tile == Tile::Visited)
        .count() as u64
}

pub fn part2(input: &Parsed) -> u64 {
    let mut map = input.clone();
    map.walk();

    map.tiles
        .into_par_iter()
        .enumerate()
        .filter(|(_, tile)| *tile == Tile::Visited)
        .map(|(i, _)| i)
        .filter(|new_obstacle_index| {
            if *new_obstacle_index == input.character_position {
                return false;
            }

            let mut map = input.clone();
            map.tiles[*new_obstacle_index] = Tile::Obstructed;

            !map.walk()
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 41)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, 6)
    }
}
