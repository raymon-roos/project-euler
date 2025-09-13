use std::{fmt::Debug, time::Instant};

fn main() {
    println!("My original");
    dbg_bench(|| largest_prime_factor(600851475143).unwrap(), 3);

    println!("recursive");
    dbg_bench(|| oh_no_what_even(600851475143), 3);

    println!("Classmate's");
    dbg_bench(|| classmates_better_version(600851475143), 3);
}

fn largest_prime_factor(n: u64) -> Option<u64> {
    let limit = f32::sqrt(n as f32) as u64;

    (2..=limit).rfind(|&i| n % i == 0 && is_prime(i))
}

fn is_prime(x: u64) -> bool {
    let limit = f32::sqrt(x as f32) as u64;

    x % 2 != 0 && (3..=limit).step_by(2).all(|i| x % i != 0)
}

fn oh_no_what_even(n: u64) -> u64 {
    fn f(n: u64, factor: u64) -> u64 {
        match n {
            2.. if factor.pow(2) > n => n,
            1 => factor,
            n if n % factor == 0 && is_prime(factor) => f(n / factor, factor),
            _ if factor == 2 => f(n, factor + 1),
            _ => f(n, factor + 2),
        }
    }
    f(n, 2)
}

fn classmates_better_version(n: u64) -> u64 {
    let mut value = n;
    let mut factor: u64 = 2;

    while value > 1 && factor.pow(2) <= value {
        match value {
            n if n % factor == 0 => value /= factor,
            _ => factor += if factor > 2 { 2 } else { 1 },
        }
    }

    match value {
        1 => factor,
        _ => value,
    }
}

fn dbg_bench<T: Debug>(func: fn() -> T, count: u8) {
    let now = Instant::now();
    for _ in 1..count {
        func();
    }
    let result = func();
    println!(
        "result: {:?}, runs: {count}, total duration: {:.4?}, avg duration: {:.4?}",
        result,
        now.elapsed(),
        now.elapsed() / count.into()
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_primes() {
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
    }

    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(29, largest_prime_factor(13195).unwrap());
    }
}
