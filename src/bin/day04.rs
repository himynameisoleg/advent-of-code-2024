//! Day 4: Ceres Search

use std::{char, error::Error, usize};

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}
// Naive Approach
fn naive(input: &str) -> i32 {
    let matrix = parse(input);
    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut count = 0;

    let dirs = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            for &(row_inc, col_inc) in &dirs {
                let mut valid = true;
                for i in 0..xmas.len() {
                    let new_row = row as isize + i as isize * row_inc;
                    let new_col = col as isize + i as isize * col_inc;
                    if new_row < 0
                        || new_row >= matrix.len() as isize
                        || new_col < 0
                        || new_col >= matrix[row].len() as isize
                    {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    let mut match_found = true;
                    for i in 0..xmas.len() {
                        let new_row = (row as isize + i as isize * row_inc) as usize;
                        let new_col = (col as isize + i as isize * col_inc) as usize;

                        if matrix[new_row][new_col] != xmas[i] {
                            match_found = false;
                            break;
                        }
                    }

                    if match_found {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

// fn naive2(input: &str) -> i32 {}
//
// // Suggested AI improvements to my naive Approach
// fn naive_ai(input: &str) -> i32 {}

// Solution from community
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

fn community(mut input: &str) -> u32 {
    let grid = parse_input(input);

    (0..grid.len())
        .map(|y| {
            (0..grid[0].len())
                .map(|x| count_xmas_starting_at_point(&grid, y as i32, x as i32))
                .sum::<u32>()
        })
        .sum()
}

fn count_xmas_starting_at_point(grid: &[Vec<u8>], y: i32, x: i32) -> u32 {
    if grid[y as usize][x as usize] != b'X' {
        return 0;
    }

    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let mut yy = y + dy;
            let mut xx = x + dx;
            let mut remaining: &[u8] = b"MAS";

            while !remaining.is_empty()
                && (0..grid.len() as i32).contains(&yy)
                && (0..grid[0].len() as i32).contains(&xx)
                && grid[yy as usize][xx as usize] == remaining[0]
            {
                yy += dy;
                xx += dx;
                remaining = &remaining[1..];
            }

            if remaining.is_empty() {
                count += 1;
            }
        }
    }

    count
}

// fn community2(mut input: &str) -> i32 {}
//
fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(naive, naive)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day04.txt");

    #[test]
    fn test_naive() {
        assert_eq!(18, naive(SAMPLE_INPUT));
    }
}
