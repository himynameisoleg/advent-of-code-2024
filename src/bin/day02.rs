//! Day 2: Red-Nosed Reports

fn day2(reports: Vec<Vec<i32>>) -> i32 {
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

fn day2_better(reports: Vec<Vec<i32>>) -> i32 {
    let mut num_safe = 0;

    for row in reports.iter() {
        for j in 1..row.len() {
            let diff = (row[j - 1] - row[j]).abs();

            if diff >= 2 {
                break;
            }

            num_safe += 1;
        }
    }

    num_safe
}

fn day2_best(reports: Vec<Vec<i32>>) -> usize {
    reports
        .into_iter()
        .filter(|report| is_safe_report(report))
        .count()
}

fn is_safe_report(report: &[i32]) -> bool {
    if report.len() <= 1 {
        return true;
    }

    let sign = (report[1] - report[0]).signum();
    report.windows(2).all(|window| {
        let diff = window[1] - window[0];
        diff.signum() == sign && (1..=3).contains(&diff.abs())
    })
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_report() -> Vec<Vec<i32>> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    #[test]
    fn test_day2() {
        let report = get_report();

        assert_eq!(day2(report), 2);
    }

    #[test]
    fn test_day2_better() {
        let report = get_report();

        assert_eq!(day2_better(report), 2);
    }

    #[test]
    fn test_day2_best() {
        let report = get_report();

        assert_eq!(day2_best(report), 2);
    }
}
