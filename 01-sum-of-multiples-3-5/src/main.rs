use dbg_bench::dbg_bench;

fn main() {
    dbg_bench(
        "sum of multiples".to_string(),
        || sum_of_multiples(&[3, 5], 1e7 as u32).unwrap(),
        5,
    );
}

fn sum_of_multiples(factors: &[u32], exclusive_limit: u32) -> Result<u128, String> {
    if factors.contains(&0) {
        return Err("Cannot divide by zero!".to_string());
    }

    Ok((1..exclusive_limit as u128)
        .filter(|&i| factors.iter().any(|&f| i % f as u128 == 0))
        .sum::<u128>())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn error_when_out_of_range() {
        assert!(sum_of_multiples(&[0], 10).is_err());
    }

    #[test]
    fn sum_of_multiples_of_2_below_10() {
        assert_eq!(Ok(0), sum_of_multiples(&[2], 0));
        assert_eq!(Ok(20), sum_of_multiples(&[2], 10))
    }

    #[test]
    fn sum_of_multiples_of_3_and_5_below_10() {
        assert_eq!(Ok(23), sum_of_multiples(&[3, 5], 10))
    }
}
