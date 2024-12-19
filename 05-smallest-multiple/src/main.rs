fn main() {
    dbg_bench("Step by n".to_string(), || lcm_step_by_n(19), 4);

    dbg_bench(
        "step by prime product".to_string(),
        || lcm_step_by_prime_product(20),
        4,
    );
}

fn lcm_step_by_n(n: usize) -> usize {
    match n {
        i @ ..=2 => i,
        _ => (n..)
            .step_by(n)
            .find(|&i| (2..=n).all(|d| i % d == 0))
            .unwrap(),
    }
}

fn lcm_step_by_prime_product(n: usize) -> usize {
    match n {
        i @ ..=2 => i,
        _ => {
            let prime_product: usize = (2..=n).filter(|&i| is_prime(i)).product();
            (prime_product..)
                .step_by(prime_product)
                .find(|&i| (2..=n).all(|d| i % d == 0))
                .unwrap()
        }
    }
}

fn is_prime(n: usize) -> bool {
    n == 2 || (n % 2 != 0 && (3..=f32::sqrt(n as f32) as usize).all(|i| n % i != 0))
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
        elapsed / count as u32
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_is_prime() {
        assert!(!is_prime(8));
        assert!(is_prime(7));
        assert!(!is_prime(6));
        assert!(is_prime(5));
    }

    #[test]
    fn correct_smallest_multiple_of_1_through_n_step_by_prime_product() {
        assert_eq!(6, lcm_step_by_n(3));
        assert_eq!(12, lcm_step_by_n(4));
        assert_eq!(60, lcm_step_by_n(6));
        assert_eq!(840, lcm_step_by_n(8));
        assert_eq!(2520, lcm_step_by_n(10));
    }
}
