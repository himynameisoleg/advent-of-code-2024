//! Day 2: Red-Nosed Reports

use std::error::Error;

fn parse_input(input: &str) -> impl Iterator<Item = Vec<i32>> + use<'_> {
    input.lines().filter(|line| !line.is_empty()).map(|line| {
        line.split(' ')
            .map(|level| level.parse::<i32>().unwrap())
            .collect()
    })
}

fn naive(input: &str) -> i32 {
    // NOTE: incorrect answer for full input
    let reports: Vec<Vec<i32>> = parse_input(input).collect();

    let mut num_safe = 0;

    for (i, row) in reports.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if j == 0 {
                continue;
            }

            let diff = (row[j - 1] - row[j]).abs();

            if diff >= 2 {
                break;
            }

            num_safe += 1;
        }
    }

    num_safe
}

fn naive2(input: &str) -> i32 {
    parse_input(input)
        .filter(|reports| {
            is_safe_report(reports)
                || (0..reports.len()).any(|i| {
                    let mut reports_with_skip = reports.clone();
                    reports_with_skip.remove(i);
                    is_safe_report(&reports_with_skip)
                })
        })
        .count() as i32
}

fn naive_ai(input: &str) -> i32 {
    let reports: Vec<Vec<i32>> = parse_input(input).collect();

    reports
        .iter()
        .filter(|report| is_safe_report(report))
        .count() as i32
}

fn is_safe_report(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..report.len() {
        let diff = (report[i] - report[i - 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }

        if report[i] > report[i - 1] {
            decreasing = false;
        } else if report[i] < report[i - 1] {
            increasing = false
        }
    }

    increasing || decreasing
}

fn community(input: &str) -> usize {
    parse_input(input)
        .filter(|levels| levels_valid(levels))
        .count()
}

fn community2(input: &str) -> usize {
    parse_input(input)
        .filter(|levels| {
            levels_valid(levels)
                || (0..levels.len()).any(|i| {
                    let mut levels_with_skip = levels.clone();
                    levels_with_skip.remove(i);
                    levels_valid(&levels_with_skip)
                })
        })
        .count()
}

fn levels_valid(levels: &[i32]) -> bool {
    if levels.len() <= 1 {
        return true;
    }

    let sign = (levels[1] - levels[0]).signum();
    levels.windows(2).all(|window| {
        let diff = window[1] - window[0];
        diff.signum() == sign && (1..=3).contains(&diff.abs())
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(naive2, community2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day02.txt");

    #[test]
    fn test_naive() {
        assert_eq!(2, naive(SAMPLE_INPUT));
    }

    #[test]
    fn test_naive_ai() {
        assert_eq!(2, naive_ai(SAMPLE_INPUT));
    }

    #[test]
    fn test_community() {
        assert_eq!(2, community(SAMPLE_INPUT));
    }
}
