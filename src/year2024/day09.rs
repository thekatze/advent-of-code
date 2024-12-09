pub struct Filesystem(Vec<Option<u64>>);

pub fn parse(input: &str) -> Filesystem {
    Filesystem(
        input
            .lines()
            .next()
            .expect("single line must exist")
            .chars()
            .enumerate()
            .flat_map(|(i, char)| {
                let is_block = (i % 2) == 0;
                let block_id = (i as u64) / 2;
                let count = char.to_digit(10).expect("number");

                // TODO: move is_block check out of loop
                (0..count).map(move |_| is_block.then_some(block_id))
            })
            .collect(),
    )
}

impl Filesystem {
    fn to_defragmented(&self) -> Self {
        let mut defragmented = self.0.clone();
        let mut left_index = 0;
        let mut right_index = self.0.len() - 1;

        while left_index <= right_index {
            while defragmented[left_index].is_some() {
                left_index += 1;
            }

            while defragmented[right_index].is_none() {
                right_index -= 1;
            }

            defragmented.swap(left_index, right_index);
        }

        // we swap one too many time at the end. unswap
        defragmented.swap(left_index, right_index);

        Filesystem(defragmented)
    }

    fn to_defragmented_files(&self) -> Self {
        todo!("this doesnt work yet");

        // let mut defragmented = self.0.clone();
        // let mut next_file_id = defragmented
        //     .iter()
        //     .rev()
        //     .find_map(|value| *value)
        //     .expect("at least one file should exist");

        // for file_id in (0..next_file_id).rev() {
        //     let file_start = defragmented
        //         .iter()
        //         .position(|value| value.map(|file| file == file_id).unwrap_or(false))
        //         .expect("file must exist");

        //     let file_length = defragmented
        //         .iter()
        //         .skip(file_start)
        //         .position(|value| value.map(|file| file != file_id).unwrap_or(true))
        //         .unwrap_or(defragmented.len());

        //     let mut free_space_index = 0;
        //     while free_space_index < file_start {
        //         while defragmented[free_space_index].is_some() {
        //             free_space_index += 1;
        //         }

        //         let free_space_len = defragmented
        //             .iter()
        //             .skip(free_space_index)
        //             .position(|value| value.is_some())
        //             .unwrap_or(defragmented.len());

        //         if (file_length < free_space_len && free_space_index < file_start) {
        //             println!("Moving file {file_id} to {free_space_index}");
        //             dbg!(&defragmented);
        //             for offset in (0..file_length) {
        //                 defragmented.swap(free_space_index + offset, file_start + offset);
        //             }
        //         } else {
        //             free_space_index += free_space_len + 1;
        //         }
        //     }
        // }

        // Filesystem(defragmented)
    }

    fn checksum(&self) -> u64 {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, block)| block.map(|id| id * i as u64))
            .sum()
    }
}

pub fn part1(input: &Filesystem) -> u64 {
    input.to_defragmented().checksum()
}
pub fn part2(input: &Filesystem) -> u64 {
    input.to_defragmented_files().checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"2333133121414131402";

    #[test]
    fn part1() {
        let result = super::part1(&parse(SAMPLE_INPUT));
        assert_eq!(result, 1928)
    }

    #[test]
    fn part2() {
        let result = super::part2(&parse(SAMPLE_INPUT));
        assert_eq!(result, 2858)
    }
}
