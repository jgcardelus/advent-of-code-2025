//! Notes:
//! - Forklifts can only access a roll of paper if there are fewer than 4 rolls of paper in the adjacent 8 positions
//!
//! Examples:
//! 0123456789
//! ..@@.@@@@.
//!
//! ..@@.@@@@.
//! @@@.@.@.@@
//! @@@@@.@.@@
//!
//! - Adjacent is corners
//!

use std::{fs::File, io::Read, path::Path};

#[derive(Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Roll,
    ToRemove,
}

impl From<Cell> for i32 {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Empty => 0,
            Cell::Roll => 1,
            Cell::ToRemove => 1,
        }
    }
}

fn access_grid_with_bounds(grid: &Vec<Vec<Cell>>, i: i32, j: i32) -> Cell {
    match (i, j) {
        (i, _) if i < 0 => Cell::Empty,
        (i, _) if (i as usize) >= grid.len() => Cell::Empty,
        (_, j) if j < 0 => Cell::Empty,
        (_, j) if (j as usize) >= grid[i as usize].len() => Cell::Empty,
        _ => grid[i as usize][j as usize].clone(),
    }
}

fn is_removable(grid: &Vec<Vec<Cell>>, i: usize, j: usize) -> bool {
    let row = i as i32;
    let col = j as i32;
    let mut acc = 0;

    for i in -1..2 {
        for j in -1..2 {
            if j == 0 && i == 0 {
                continue;
            }
            acc += i32::from(access_grid_with_bounds(grid, row + i, col + j))
        }
    }

    return acc < 4;
}

pub fn get_accessable_rolls_with_removal(grid: &mut Vec<Vec<Cell>>) -> u32 {
    let mut moved_rolls = access_and_remove_rolls(grid);
    let mut new_moved_rolls = moved_rolls;

    while new_moved_rolls > 0 {
        new_moved_rolls = access_and_remove_rolls(grid);
        moved_rolls += new_moved_rolls;
    }

    moved_rolls
}

fn access_and_remove_rolls(grid: &mut Vec<Vec<Cell>>) -> u32 {
    let moved_rolls = get_accessable_rolls(
        grid,
        Some(|grid: &mut Vec<Vec<Cell>>, row: usize, col: usize| {
            grid[row][col] = Cell::ToRemove;
        }),
    );

    map_grid(grid, &mut |grid: &mut Vec<Vec<Cell>>,
                         row: usize,
                         col: usize| {
        if grid[row][col] == Cell::ToRemove {
            grid[row][col] = Cell::Empty;
        }
    });

    moved_rolls
}

fn map_grid<Transform>(grid: &mut Vec<Vec<Cell>>, transform: &mut Transform)
where
    Transform: FnMut(&mut Vec<Vec<Cell>>, usize, usize),
{
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            transform(grid, row, col)
        }
    }
}

pub fn get_accessable_rolls<OnRemove>(
    grid: &mut Vec<Vec<Cell>>,
    mut on_remove: Option<OnRemove>,
) -> u32
where
    OnRemove: FnMut(&mut Vec<Vec<Cell>>, usize, usize),
{
    let mut moved_rolls = 0;

    map_grid(grid, &mut |grid: &mut Vec<Vec<Cell>>,
                         row: usize,
                         col: usize| {
        if grid[row][col] != Cell::Roll {
            return;
        }

        if is_removable(grid, row, col) {
            moved_rolls += 1;
            if let Some(on_remove) = on_remove.as_mut() {
                on_remove(grid, row, col)
            }
        }
    });

    moved_rolls
}

fn rolls_to_grid(rolls: &str) -> Vec<Vec<Cell>> {
    rolls
        .trim()
        .lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .split("")
                .filter(|char| !char.is_empty())
                .map(|char| match char {
                    "." => Cell::Empty,
                    "@" => Cell::Roll,
                    other => panic!("Rolls can only be made of '.' or '@', got '{}'", other),
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>()
}

pub fn read_rolls(path: &Path) -> Vec<Vec<Cell>> {
    let mut file = File::open(path).expect("Couldn't open file");
    let mut rolls = String::new();

    file.read_to_string(&mut rolls)
        .expect("Couldn't read contents");

    rolls_to_grid(&rolls)
}

#[cfg(test)]
mod test {
    use crate::d04_printing::{
        get_accessable_rolls, get_accessable_rolls_with_removal, rolls_to_grid,
    };

    #[test]
    fn test_get_accessable_rolls_valid() {
        let rolls = "
		     ..@@.@@@@.
		     @@@.@.@.@@
		     @@@@@.@.@@
		     @.@@@@..@.
		     @@.@@@@.@@
		     .@@@@@@@.@
		     .@.@.@.@@@
		     @.@@@.@@@@
		     .@@@@@@@@.
		     @.@.@@@.@.
	     ";

        let mut grid = rolls_to_grid(rolls);
        let result = get_accessable_rolls::<fn(&mut Vec<Vec<_>>, usize, usize)>(&mut grid, None);

        assert_eq!(13, result, "Expected {}, got {}", 13, result);
    }

    #[test]
    fn test_get_accessable_rolls_with_removal_valid() {
        let rolls = "
		     ..@@.@@@@.
		     @@@.@.@.@@
		     @@@@@.@.@@
		     @.@@@@..@.
		     @@.@@@@.@@
		     .@@@@@@@.@
		     .@.@.@.@@@
		     @.@@@.@@@@
		     .@@@@@@@@.
		     @.@.@@@.@.
	     ";

        let mut grid = rolls_to_grid(rolls);
        let result = get_accessable_rolls_with_removal(&mut grid);

        assert_eq!(43, result, "Expected {}, got {}", 43, result);
    }
}
