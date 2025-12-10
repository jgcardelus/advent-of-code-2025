//! Problem input
//! 	11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
//! 	1698522-1698528,446443-446449,38593856-38593862,565653-565659,
//! 	824824821-824824827,2121212118-2121212124
//!
//!
//! Shape
//!
//! One long, comma-separated line
//! Ranges <start>-<end>
//! Invalid pattern's are sequences of digits repeated *twice*
//! 55, 6464 123123
//!
//!
//!
//! Todo:
//! - [x] Invalid is made only of some sequence of digits repeated at least twice
//! - [x] Update to use depedency injection

use std::{fs::File, io::Read, ops::Range, path::Path};

use anyhow::bail;

pub fn read_ranges(path: &Path) -> anyhow::Result<Vec<Range<u64>>> {
    let mut file = File::open(path)?;
    let mut ranges = String::new();
    file.read_to_string(&mut ranges)?;

    let ranges: anyhow::Result<Vec<Range<u64>>> = ranges
        .trim()
        .split(",")
        .into_iter()
        .map(|range| {
            let range = range.split("-").collect::<Vec<&str>>();
            match (range.get(0), range.get(1)) {
                (Some(start), Some(end)) => Ok(Range {
                    start: start.parse::<u64>()?,
                    end: end.parse::<u64>()?,
                }),
                _ => bail!("Invalid range!"),
            }
        })
        .fold(Ok(Vec::new()), |acc, range| {
            let mut vec = match acc {
                Ok(vec) => vec,
                other => return other,
            };

            match range {
                Ok(range) => vec.push(range),
                Err(error) => return Err(error),
            }

            Ok(vec)
        });

    ranges
}

// 565656 -> 56, 56, 56
// Should check from smaller to bigger
pub fn is_repeating_sequence(number: &str) -> bool {
    // Find all the windows sizes where you could have at least to seqs
    // 111111 -> 3, 2, 1
    let mut window_sizes: Vec<usize> = Vec::new();
    let max_window_size = number.len() / 2;
    for window_size in 1..(max_window_size + 1) {
        if number.len() % window_size != 0 {
            // The window doesn't fit neatly into the number
            continue;
        }

        window_sizes.push(window_size);
    }

    let is_repeating_sequence = window_sizes
        .iter()
        .any(|window_size| is_repeating_sequence_for_window(&number, *window_size));

    return is_repeating_sequence;
}

fn is_repeating_sequence_for_window(number: &str, window_size: usize) -> bool {
    let pivot = number
        .get(0..window_size)
        .expect("window_size must be smaller than number");

    let slices = number.len() / window_size;

    for i in 1..slices {
        let start = window_size * i;
        let end = start + window_size;
        let check = number
            .get(start..end)
            .expect("window slice must fit into number");

        if pivot != check {
            return false;
        }
    }

    return true;
}

pub fn is_sequence_twice(number: &str) -> bool {
    if number.len() % 2 != 0 {
        return false;
    }
    let number = number.chars().collect::<Vec<char>>();
    let splitted = number.split_at_checked(number.len() / 2);

    match splitted {
        Some((first, second)) => {
            for (a, b) in first.iter().zip(second) {
                if a != b {
                    return false;
                }
            }
            return true;
        }
        _ => return false,
    }
}

pub fn find_invalid_ids(range: &Range<u64>, is_invalid_id: fn(&str) -> bool) -> u64 {
    (range.start..(range.end + 1))
        .into_iter()
        .map(|number| (number, number.to_string()))
        .filter(|(_, number)| is_invalid_id(number))
        .fold(0, |acc, (number, _)| acc + number)
}

pub fn find_invalid_ids_of_ranges(
    ranges: &[Range<u64>],
    is_invalid_id: fn(&str) -> bool,
) -> Option<u64> {
    ranges
        .iter()
        .map(|range| find_invalid_ids(range, is_invalid_id))
        .reduce(|acc, sum| acc + sum)
}

#[cfg(test)]
mod test {
    use std::ops::Range;

    use crate::d02_gift_shop::{
        find_invalid_ids, is_repeating_sequence, is_repeating_sequence_for_window,
        is_sequence_twice,
    };

    fn test_find_invalid_ids_part_one() {
        let ranges = vec![
            (Range { start: 11, end: 12 }, 11),
            (Range { start: 21, end: 22 }, 22),
            (
                Range {
                    start: 998,
                    end: 1012,
                },
                1010,
            ),
            (
                Range {
                    start: 1188511880,
                    end: 1188511890,
                },
                1188511885,
            ),
            (
                Range {
                    start: 222220,
                    end: 222224,
                },
                222222,
            ),
            (
                Range {
                    start: 1698522,
                    end: 1698528,
                },
                0,
            ),
        ];

        for (range, expect) in ranges {
            let result = find_invalid_ids(&range, is_sequence_twice);
            assert_eq!(result, expect)
        }
    }

    #[test]
    fn test_is_repeating_sequence_valid() {
        let tests = vec![
            ("565656", true),
            ("111111", true),
            ("333333", true),
            ("334334", true),
            ("123454", false),
            ("333334", false),
        ];

        tests.into_iter().for_each(|(number, expect)| {
            let result = is_repeating_sequence(number);
            assert_eq!(
                expect, result,
                "Expected {}, got {} for number {}",
                expect, result, number
            );
        });
    }

    #[test]
    fn test_is_repeating_sequence_for_window_valid() {
        let tests = vec![
            ("565656", 2usize, true),
            ("111111", 1usize, true),
            ("333333", 3usize, true),
            ("334334", 2usize, false),
            ("112112", 1usize, false),
            ("333334", 3usize, false),
        ];

        tests.into_iter().for_each(|(number, window_size, expect)| {
            let result = is_repeating_sequence_for_window(number, window_size);
            assert_eq!(
                expect, result,
                "Expected {}, got {} for number {} with window size {}",
                expect, result, number, window_size
            );
        });
    }
}
