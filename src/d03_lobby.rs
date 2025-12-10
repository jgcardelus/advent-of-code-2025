//! Notes:
//! - Turn on exactly two batteries
//! - The joltage they produce = the number formed by the digits of the batteries turned on
//! 	- In 12345 if I turn on 2 and 4 I get 24
//! - Batteries cannot be re-arranged
//! - Find max possible joltage
//! - Sum all of the joltages
//!
//!
//! Brute force -- double for-loop
//! Dynamic programming?
//! max_j(i + max_j(rest), max_j(rest))

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn find_max_joltage(batteries: &[u8], activations: usize) -> u64 {
    assert!(batteries.len() > 0, "There must be at least one battery");

    let mut max_joltages: Vec<Vec<u64>> = vec![vec![0; batteries.len()]; activations];

    for row in 0..activations {
        for (col, battery) in batteries.iter().enumerate().rev() {
            // Previous contains biggest possible (n-1) number
            let prev = match (row, col) {
                (0, _) => 0,
                (_, j) if j == batteries.len() - 1 => 0,
                _ => max_joltages[row - 1][col + 1],
            };

            // Right contains current biggest possible n number
            let right = match col {
                j if j == batteries.len() - 1 => 0,
                _ => max_joltages[row][col + 1],
            };

            let exponent: u32 = if prev == 0 { 0 } else { prev.ilog10() + 1 };
            let current = (*battery as u64) * 10_u64.pow(exponent) + prev;
            max_joltages[row][col] = current.max(right);
        }
    }

    return max_joltages[activations - 1][0];
}

pub fn find_joltage_in_battery_packs(
    packs: &Vec<Vec<u8>>,
    find_max_joltage: fn(&[u8]) -> u64,
) -> u64 {
    assert!(packs.len() > 0, "There must be at least one pack");

    packs
        .iter()
        .map(|batteries| find_max_joltage(batteries))
        .reduce(|acc, joltage| acc + joltage)
        .expect("No joltage was found")
}

pub fn read_battery_packs(path: &Path) -> Vec<Vec<u8>> {
    let file = File::open(path).expect("Couldn't open file");
    let reader = BufReader::new(file);

    let packs = reader
        .lines()
        .map(|pack| {
            let pack = pack.expect("Is this the actual battery file?");
            let pack = pack
                .trim()
                .split("")
                .filter(|cell| !cell.is_empty())
                .map(|battery| {
                    battery
                        .parse::<u8>()
                        .expect("Cells should only contain numbers")
                })
                .collect::<Vec<u8>>();

            pack
        })
        .filter(|pack| pack.len() > 0)
        .collect::<Vec<Vec<u8>>>();

    packs
}

#[cfg(test)]
mod test {
    use crate::d03_lobby::find_max_joltage;

    #[test]
    fn test_find_max_joltage_valid() {
        let tests = [
            ([9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2, 98),
            ([8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2, 89),
            ([2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 2, 78),
            ([8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2, 92),
            (
                [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                12,
                987654321111,
            ),
            (
                [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                12,
                811111111119,
            ),
            (
                [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                12,
                434234234278,
            ),
            (
                [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
                12,
                888911112111,
            ),
        ];

        tests
            .into_iter()
            .for_each(|(batteries, activations, expect)| {
                let result = find_max_joltage(&batteries, activations);
                assert_eq!(
                    expect, result,
                    "Expected battery to be {}, got {}. Battery pack: {:?}",
                    expect, result, batteries
                );
            });
    }
}
