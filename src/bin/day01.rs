//! Day 1: Historian Hysteria

fn day1(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut distance = 0;
    for (i, _) in left.iter().enumerate() {
        let diff = left[i] - right[i];
        distance += diff;
    }

    distance.abs()
}

fn day1_improved(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let distance: i32 = left.iter().zip(right.iter()).map(|(l, r)| l - r).sum();

    distance.abs()
}

fn day1_improved_alt(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let s = day1(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(s, 11);
    }

    #[test]
    fn test_day1_improved() {
        let s = day1_improved(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(s, 11);
    }

    #[test]
    fn test_day1_improved_alt() {
        let s = day1_improved_alt(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(s, 11);
    }
}
