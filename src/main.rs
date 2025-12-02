use std::path::Path;

use crate::d01_a_password::{
    get_ends_at_zero, get_number_of_passes_through_zero, read_rotations_from_file,
};

mod d01_a_password;

fn day_one() -> anyhow::Result<()> {
    let rotations = read_rotations_from_file(Path::new("./data/day-1/long.txt"))?;
    let ends_at_zero = get_ends_at_zero(&rotations);
    println!("{}", ends_at_zero);
    let passes_through_zero = get_number_of_passes_through_zero(&rotations);
    println!("{}", passes_through_zero);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    day_one()?;
    Ok(())
}
