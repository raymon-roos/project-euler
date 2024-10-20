fn main() {
    dbg_bench(
        "Iterator".to_string(),
        || smallest_multiple_with_divisors_1_through_n(21).unwrap(),
        4,
    );
}

fn smallest_multiple_with_divisors_1_through_n(n: u64) -> Option<u64> {
    match n {
        ..=1 => None,
        _ => (n..).find(|&i| (1..=n).all(|d| i % d == 0)),
    }
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
mod test {
    use super::*;

    #[test]
    fn correct_smallest_multiple_of_1_through_10() {
        assert_eq!(
            2520,
            smallest_multiple_with_divisors_1_through_n(10).unwrap()
        );
    }
}
