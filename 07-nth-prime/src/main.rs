use std::env;

fn main() {
    let n = env::args().nth(1).map_or(100, |arg| {
        arg.parse::<usize>()
            .expect("Argument should be a valid integer!")
    });

    println!("{:?}", &find_primes(n)[0..100]);
}

fn find_primes(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![];
    }

    let mut primes = vec![2];
    let mut i = 3;

    while primes.len() < n {
        if is_prime(i, &primes) {
            primes.push(i);
        }
        i += 2;
    }

    primes
}

fn is_prime(num: usize, primes: &[usize]) -> bool {
    let root = f64::sqrt(num as f64) as usize;

    !primes
        .iter()
        .take_while(|&p| *p <= root)
        .any(|p| num % p == 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero_primes() {
        let v: Vec<usize> = vec![];
        assert_eq!(v, find_primes(0));
    }

    #[test]
    fn one_prime() {
        assert_eq!(vec![2], find_primes(1));
    }

    #[test]
    fn two_primes() {
        assert_eq!(vec![2, 3], find_primes(2));
    }

    #[test]
    fn one_hundred_primes() {
        assert_eq!(
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257,
                263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353,
                359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449,
                457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541
            ],
            find_primes(100)
        );
    }
}
