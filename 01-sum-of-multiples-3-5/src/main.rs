fn main() {
    println!("{}", sum_of_multiples(3, 10).unwrap())
}

#[derive(Debug)]
struct RangeError;

fn sum_of_multiples(step: u32, inclusive_limit: u32) -> Result<u32, RangeError> {
    match (step, inclusive_limit) {
        (0, _) | (_, 0) => Err(RangeError),
        (step @ 1.., limit @ 1..) => Ok((1..limit).filter(|i| i % step == 0).sum()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn error_when_out_of_range() {
        assert!(sum_of_multiples(0, 10).is_err());
        assert!(sum_of_multiples(2, 0).is_err());
    }

    #[test]
    fn sum_of_multiples_of_2_below_10() {
        assert_eq!(20, sum_of_multiples(2, 10).unwrap())
    }

    #[test]
    fn sum_of_multiples_of_3_and_5_below_10() {
        assert_eq!(
            23,
            sum_of_multiples(3, 10).unwrap() + sum_of_multiples(5, 10).unwrap()
        )
    }
}
