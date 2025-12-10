use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::bail;
use clap::{Parser, ValueEnum, command};

use crate::{
    d01_a_password::{
        get_ends_at_zero, get_number_of_passes_through_zero, read_rotations_from_file,
    },
    d02_gift_shop::{
        find_invalid_ids_of_ranges, is_repeating_sequence, is_sequence_twice, read_ranges,
    },
    d03_lobby::{find_joltage_in_battery_packs, find_max_joltage, read_battery_packs},
};

mod d01_a_password;
mod d02_gift_shop;
mod d03_lobby;

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ProblemPart {
    One,
    Two,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Day from advent of code to run. Days are 1-index.
    #[arg(long, default_value_t = 1)]
    day: u16,

    // Part of the problem. There are two per day (AFAIK)
    #[arg(long, value_enum, default_value_t = ProblemPart::One)]
    part: ProblemPart,
}

fn day_one(part: &ProblemPart) -> anyhow::Result<()> {
    let rotations = read_rotations_from_file(Path::new("./data/day-1/long.txt"))?;
    match part {
        ProblemPart::One => {
            let ends_at_zero = get_ends_at_zero(&rotations);
            println!("{}", ends_at_zero);
        }
        ProblemPart::Two => {
            let passes_through_zero = get_number_of_passes_through_zero(&rotations);
            println!("{}", passes_through_zero);
        }
    };

    Ok(())
}

fn day_two(part: &ProblemPart) -> anyhow::Result<()> {
    let ranges = read_ranges(Path::new("./data/day-2/long.txt"))?;

    let value = match part {
        ProblemPart::One => find_invalid_ids_of_ranges(&ranges, is_sequence_twice),
        ProblemPart::Two => find_invalid_ids_of_ranges(&ranges, is_repeating_sequence),
    };

    match value {
        Some(value) => println!("{}", value),
        None => println!("Found no value"),
    };

    Ok(())
}

fn day_three(part: &ProblemPart) -> anyhow::Result<()> {
    let packs = read_battery_packs(Path::new("./data/day-3/long.txt"));

    let value = match part {
        ProblemPart::One => {
            find_joltage_in_battery_packs(&packs, |batteries| find_max_joltage(batteries, 2))
        }
        ProblemPart::Two => {
            find_joltage_in_battery_packs(&packs, |batteries| find_max_joltage(batteries, 12))
        }
    };

    println!("And joltage is.... {}", value);

    Ok(())
}

fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let start = now();

    let result = match args.day {
        0 => {
            bail!("I said 1-index! jeez")
        }
        1 => day_one(&args.part),
        2 => day_two(&args.part),
        3 => day_three(&args.part),
        _ => bail!("I'm working on it... heheheh"),
    };

    let end = now();
    let delta = end - start;

    println!("It took {} ms to run", delta);

    result
}
