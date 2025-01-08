//! Day 1: Historian Hysteria

use std::{collections::HashMap, error::Error};

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    // NOTE: incorrect answer for full input
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut split = line.split_ascii_whitespace();
            let l = split.next().unwrap().parse::<i32>().unwrap();
            let r = split.next().unwrap().parse::<i32>().unwrap();
            (l, r)
        })
        .unzip()
}

fn naive(input: &str) -> i32 {
    let (left, right) = parse_input(input);

    let mut distance = 0;
    for (i, _) in left.iter().enumerate() {
        let diff = left[i] - right[i];
        distance += diff;
    }

    distance.abs()
}

fn naive2(input: &str) -> i32 {
    let (left, right) = parse_input(input);

    let mut score = 0;

    for (i, _) in left.iter().enumerate() {
        let mut multiplier = 0;
        for (j, _) in right.iter().enumerate() {
            if right[j] == left[i] {
                multiplier += 1;
            }
        }
        score += left[i] * multiplier;
    }

    score
}

fn naive_ai(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);

    let distance: i32 = left.iter().zip(right.iter()).map(|(l, r)| l - r).sum();
    distance.abs()
}

fn naive_ai2(input: &str) -> i32 {
    let (left, right) = parse_input(input);

    let mut right_counts = HashMap::new();

    for &num in &right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let mut score = 0;
    for &num in &left {
        if let Some(&count) = right_counts.get(&num) {
            score += num * count;
        }
    }

    score
}

fn community(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(naive_ai2, naive2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day01.txt");

    #[test]
    fn test_naive() {
        assert_eq!(11, naive(SAMPLE_INPUT));
    }

    #[test]
    fn test_naive2() {
        assert_eq!(31, naive2(SAMPLE_INPUT));
    }

    #[test]
    fn test_naive_ai() {
        assert_eq!(11, naive_ai(SAMPLE_INPUT));
    }

    #[test]
    fn test_naive_ai2() {
        assert_eq!(31, naive_ai2(SAMPLE_INPUT));
    }

    #[test]
    fn test_day1_improved_alt() {
        assert_eq!(11, community(SAMPLE_INPUT));
    }
}
