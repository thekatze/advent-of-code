pub struct Parsed(String, usize, usize);

pub fn parse(input: &str) -> Parsed {
    let height = input.lines().count();
    let width = input
        .lines()
        .next()
        .expect("one line must exist")
        .chars()
        .count();

    Parsed(input.replace("\n", ""), width, height)
}

pub fn part1(input: &Parsed) -> u64 {
    const NEEDLE_LEN: usize = 4;
    let needles = ["XMAS", "SAMX"];

    let (grid, width, height) = (&input.0, input.1, input.2);

    (0..height)
        .flat_map(|y| (0..width).map(move |x| (y, x)))
        .map(|(y, x)| {
            let index = y * width + x;

            let mut count = 0;

            if x <= width - NEEDLE_LEN {
                let horizontal = &grid[index..index + NEEDLE_LEN];
                if needles.contains(&horizontal) {
                    count += 1;
                }
            }

            if y <= height - NEEDLE_LEN {
                let vertical = &grid
                    .chars()
                    .skip(index)
                    .step_by(width)
                    .take(NEEDLE_LEN)
                    .collect::<String>();
                if needles.contains(&vertical.as_str()) {
                    count += 1;
                }
            }

            if x <= width - NEEDLE_LEN && y <= height - NEEDLE_LEN {
                let diagonal = &grid
                    .chars()
                    .skip(index)
                    .step_by(width + 1)
                    .take(NEEDLE_LEN)
                    .collect::<String>();

                if needles.contains(&diagonal.as_str()) {
                    count += 1;
                }
            }

            if x >= NEEDLE_LEN - 1 && y <= height - NEEDLE_LEN {
                let antidiagonal = &grid
                    .chars()
                    .skip(index)
                    .step_by(width - 1)
                    .take(NEEDLE_LEN)
                    .collect::<String>();

                if needles.contains(&antidiagonal.as_str()) {
                    count += 1;
                }
            }

            count
        })
        .sum()
}

pub fn part2(input: &Parsed) -> u64 {
    let (grid, width, height) = (&input.0, input.1, input.2);

    // as_bytes because we're not doing unicode shenanigans here
    let grid = grid.as_bytes();

    let top_left_offset: isize = -1 - (width as isize);
    let bottom_left_offset: isize = -1 + (width as isize);
    let top_right_offset: isize = 1 - (width as isize);
    let bottom_right_offset: isize = 1 + (width as isize);

    (1..height - 1)
        .flat_map(|y| (1..width - 1).map(move |x| (y, x)))
        .map(|(y, x)| {
            let center = y * width + x;

            if grid[center] != b'A' {
                return 0;
            }

            let diagonal: [u8; 2] = [
                grid[(center as isize + top_left_offset) as usize],
                grid[(center as isize + bottom_right_offset) as usize],
            ];

            let antidiagonal: [u8; 2] = [
                grid[(center as isize + top_right_offset) as usize],
                grid[(center as isize + bottom_left_offset) as usize],
            ];

            if (diagonal[0] == b'M' && diagonal[1] == b'S'
                || diagonal[0] == b'S' && diagonal[1] == b'M')
                && (antidiagonal[0] == b'M' && antidiagonal[1] == b'S'
                    || antidiagonal[0] == b'S' && antidiagonal[1] == b'M')
            {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 18)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, 9)
    }
}
