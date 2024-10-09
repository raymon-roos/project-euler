fn main() {
    println!("{}", largest_prime_factor(13195).unwrap());

    // Don't run this unless you're willing to wait a while...
    // println!(
    //     "{}",
    //     largest_prime_factor(600851475143).unwrap()
    // );
}

fn largest_prime_factor(n: u64) -> Option<usize> {
    find_primes_up_to(n)
        .into_iter()
        .rev()
        .find(|&p| n % p as u64 == 0)
}

fn find_primes_up_to(inclusive_limit: u64) -> Vec<usize> {
    let mut primes = vec![2];

    (3..=inclusive_limit).step_by(2).for_each(|i| {
        if is_prime(i as usize, primes.as_slice()) {
            primes.push(i as usize)
        }
    });

    primes
}

fn is_prime(x: usize, primes: &[usize]) -> bool {
    let root = f32::sqrt(x as f32) as usize;
    !primes
        .iter()
        .take_while(|&p| *p <= root)
        .any(|p| x % p == 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_primes() {
        assert_eq!(
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29],
            find_primes_up_to(29)
        );
    }

    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(29, largest_prime_factor(13195).unwrap());
    }
}
