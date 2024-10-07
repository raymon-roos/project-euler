fn main() {
    println!("{}", sum_even_fibonacci_below_4e6());
    println!("{}", fib_alternate());
}

// Naive version
fn sum_even_fibonacci_below_4e6() -> u32 {
    fn f(even_sum: u32, fib: u32, prev: u32) -> u32 {
        match fib {
            ..4_000_000 if fib % 2 == 0 => f(even_sum + fib, fib + prev, fib),
            ..4_000_000 => f(even_sum, fib + prev, fib),
            _ => even_sum,
        }
    }
    f(0, 1, 1)
}

// Clever version I found on stack overflow:
// https://stackoverflow.com/a/3271744
// No doubt you could proof it somehow, but that's beyond me
fn fib_alternate() -> u32 {
    fn f(sum: u32, even_fib: u32, prev: u32) -> u32 {
        match even_fib {
            ..4_000_000 => f(sum + even_fib, even_fib * 4 + prev, even_fib),
            _ => sum,
        }
    }
    f(0, 0, 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sum_first_ten_even_fib() {
        assert_eq!(4613732, sum_even_fibonacci_below_4e6());
        assert_eq!(4613732, fib_alternate());
    }
}
