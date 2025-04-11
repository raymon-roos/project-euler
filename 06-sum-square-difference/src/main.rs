fn main() {
    dbg_bench(
        "simple".to_string(),
        || sum_square_difference(100).unwrap(),
        5,
    );
    dbg_bench(
        "clever".to_string(),
        || fast_sum_square_difference(100).unwrap(),
        5,
    );
}

fn sum_square_difference(n: usize) -> Option<usize> {
    if n < 1 {
        return None;
    }

    let square_of_sums = (1..=n).sum::<usize>().pow(2);
    let sum_of_squares: usize = (1..=n).map(|i| i * i).sum();

    Some(square_of_sums - sum_of_squares)
}

fn fast_sum_square_difference(n: isize) -> Option<isize> {
    if n < 1 {
        return None;
    }

    // Don't know why the result is negative, that isn't part
    // of the answer at https://stackoverflow.com/a/15593495
    Some(-(3 * n.pow(2) + 2 * n) * (1 - n.pow(2)) / 12)
}

fn dbg_bench<T: std::fmt::Display>(title: String, func: fn() -> T, count: u8) {
    let now = std::time::Instant::now();
    for _ in 1..count {
        func();
    }
    let result = func();
    let elapsed = now.elapsed();

    println!(
        "{title} -> result: {result}, runs: {count}, total duration: {:.4?}, avg duration: {:.4?}",
        elapsed,
        elapsed / 4
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_sum_square_difference() {
        assert_eq!(2640, sum_square_difference(10).unwrap());
        assert_eq!(2640, fast_sum_square_difference(10).unwrap());
    }
}
