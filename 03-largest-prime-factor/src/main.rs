fn main() {
    println!("{}", largest_prime_factor(600851475143).unwrap());
}

fn largest_prime_factor(n: u64) -> Option<u64> {
    let limit = f32::sqrt(n as f32) as u64;

    (2..=limit).rev().find(|&i| n % i == 0 && is_prime(i))
}

fn is_prime(x: u64) -> bool {
    let limit = f32::sqrt(x as f32) as usize;

    x % 2 != 0 && !(3..=limit).step_by(2).any(|i| x % i as u64 == 0)
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
