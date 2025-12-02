// Rotations L (lower numbers),R (higher numbers)
// Rotations have a number
// 11 + R18 -> 19 + L19 -> 0
// If the dial reaches 99, the next will be 0
// 95 + R5 -> 0
//
// The actual password is how many times the dial is pointing to zero
// NOTE to Self: Optimize using division and module duh

use std::{fmt, fs::File, io::Read, path::Path};

use anyhow::Context;

const STARTING_POSITION: i64 = 50;
const WRAP_AROUND: i64 = 99;

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct Rotation {
    direction: Direction,
    clicks: i64,
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.direction {
            Direction::Left => write!(f, "L{}", self.clicks)?,
            Direction::Right => write!(f, "R{}", self.clicks)?,
        };

        Ok(())
    }
}

pub fn parse_rotations(contents: &str) -> Vec<Rotation> {
    contents
        .split("\n")
        .filter_map(|raw_rotation| match raw_rotation.trim() {
            "" => None,
            raw_rotation => Some(raw_rotation),
        })
        .filter_map(|raw_rotation| {
            let mut chars = raw_rotation.chars();

            let direction = match chars.nth(0) {
                Some('L') => Direction::Left,
                Some('R') => Direction::Right,
                _ => return None,
            };

            let clicks = match chars.as_str().parse::<i64>() {
                Ok(clicks) => clicks,
                Err(_) => return None,
            };

            Some(Rotation { direction, clicks })
        })
        .collect::<Vec<Rotation>>()
}

pub fn read_rotations_from_file(path: &Path) -> anyhow::Result<Vec<Rotation>> {
    let mut file = File::open(path).context("Couldn't open file")?;
    let mut contents: String = String::new();

    file.read_to_string(&mut contents)
        .context("Couldn't read contents of file")?;

    let rotations = parse_rotations(&contents);

    Ok(rotations)
}

pub fn get_number_of_passes_through_zero(rotations: &Vec<Rotation>) -> i64 {
    let mut zero_passes: i64 = 0;
    let mut current_position: i64 = STARTING_POSITION;

    for rotation in rotations {
        match rotation.direction {
            Direction::Left => {
                if current_position == 0 {
                    // Pass through zero was already counted
                    // And it's going to be counted again in while-loop
                    zero_passes -= 1;
                }

                current_position -= rotation.clicks;

                while current_position < 0 {
                    zero_passes += 1;
                    current_position += WRAP_AROUND + 1;
                }

                if current_position == 0 {
                    zero_passes += 1;
                }
            }
            Direction::Right => {
                current_position += rotation.clicks;
                while current_position > WRAP_AROUND {
                    // Zero counts as value
                    zero_passes += 1;
                    current_position -= WRAP_AROUND + 1;
                }
            }
        };
    }

    return zero_passes;
}

pub fn get_ends_at_zero(rotations: &Vec<Rotation>) -> i64 {
    let mut zero_passes: i64 = 0;
    let mut current_position: i64 = STARTING_POSITION;

    for rotation in rotations {
        match rotation.direction {
            Direction::Left => {
                current_position -= rotation.clicks;

                while current_position < 0 {
                    current_position += WRAP_AROUND + 1;
                }
            }
            Direction::Right => {
                current_position += rotation.clicks;
                while current_position > WRAP_AROUND {
                    // Zero counts as value
                    current_position -= WRAP_AROUND + 1;
                }
            }
        };

        if current_position == 0 {
            zero_passes += 1;
        }
    }

    return zero_passes;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_rotations() {
        let rotation = "
        	L68
         	L30
          	R48

     	";

        let expected = vec![
            Rotation {
                direction: Direction::Left,
                clicks: 68,
            },
            Rotation {
                direction: Direction::Left,
                clicks: 30,
            },
            Rotation {
                direction: Direction::Right,
                clicks: 48,
            },
        ];

        let results = parse_rotations(rotation);

        assert_eq!(results, expected)
    }

    #[test]
    fn test_get_passes_through_zero() {
        let contents = "
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
	     ";

        let rotations = parse_rotations(contents);

        let result = get_number_of_passes_through_zero(&rotations);

        assert_eq!(result, 6)
    }

    #[test]
    fn test_get_ends_at_zero() {
        let contents = "
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
	     ";

        let rotations = parse_rotations(contents);

        let result = get_ends_at_zero(&rotations);

        assert_eq!(result, 3)
    }

    #[test]
    fn test_multiple_passes_through_zero_in_same_rotation() {
        let contents = "
        R1000
        L1000
	    ";

        let rotations = parse_rotations(contents);

        let result = get_number_of_passes_through_zero(&rotations);

        assert_eq!(result, 20)
    }
}
