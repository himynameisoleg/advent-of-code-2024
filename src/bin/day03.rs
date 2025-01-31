//! Day 3: Mull It Over

use std::error::Error;

use regex::Regex;
use winnow::ascii::digit1;
use winnow::combinator::{separated_pair, terminated};
use winnow::prelude::*;

// Naive Approach
fn naive(input: &str) -> i32 {
    let re = Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();

    let mut muls: Vec<i32> = vec![];

    for caps in re.captures_iter(input) {
        let x: i32 = caps["x"].parse().unwrap();
        let y: i32 = caps["y"].parse().unwrap();
        muls.push(x * y);
    }

    muls.iter().sum()
}

fn naive2(input: &str) -> i32 {
    let re =
        Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)")
            .unwrap();

    let mut enabled = true;
    let mut sum = 0;

    for caps in re.captures_iter(input) {
        if caps.name("do").is_some() {
            enabled = true;
        } else if caps.name("dont").is_some() {
            enabled = false;
        } else if caps.name("x").is_some() {
            let x: i32 = caps["x"].parse().unwrap();
            let y: i32 = caps["y"].parse().unwrap();
            if enabled {
                sum += x * y;
            }
        }
    }

    sum
}

// Suggested AI improvements to my naive Approach
fn naive_ai(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    for cap in re.captures_iter(input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        sum += x * y;
    }

    sum
}

// Solution from community
fn community(mut input: &str) -> i32 {
    let mut sum = 0;

    while input.len() >= 4 {
        if input.starts_with("mul(") {
            input = &input["mul(".len()..];
            if let Ok((l, r)) = parse_mul_suffix(&mut input) {
                sum += l * r;
            }
        } else {
            input = &input[1..];
        }
    }

    sum
}

fn community2(mut input: &str) -> i32 {
    let mut sum = 0;
    let mut enabled = true;

    while input.len() >= 4 {
        if enabled && input.starts_with("mul(") {
            input = &input["mul(".len()..];
            if let Ok((l, r)) = parse_mul_suffix(&mut input) {
                sum += l * r;
            }
        } else if input.starts_with("do()") {
            enabled = true;
            input = &input["do()".len()..];
        } else if input.starts_with("don't()") {
            enabled = false;
            input = &input["don't()".len()..];
        } else {
            input = &input[1..];
        }
    }

    sum
}

fn parse_mul_suffix(mut input: &mut &str) -> PResult<(i32, i32)> {
    terminated(separated_pair(parse_i32, ',', parse_i32), ')').parse_next(input)
}

fn parse_i32(input: &mut &str) -> PResult<i32> {
    digit1.parse_to().parse_next(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(naive2, community2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day03.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample/day03-2.txt");

    #[test]
    fn test_naive() {
        assert_eq!(161, naive(SAMPLE_INPUT));
    }

    #[test]
    fn test_naive2() {
        assert_eq!(48, naive2(SAMPLE_INPUT_2));
    }

    #[test]
    fn test_naive_ai() {
        assert_eq!(161, naive_ai(SAMPLE_INPUT));
    }

    #[test]
    fn test_community() {
        assert_eq!(161, community(SAMPLE_INPUT));
    }

    #[test]
    fn test_community2() {
        assert_eq!(48, community2(SAMPLE_INPUT_2));
    }
}
