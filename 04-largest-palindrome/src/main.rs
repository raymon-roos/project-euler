use dbg_bench::dbg_bench;

fn main() {
    dbg_bench(
        "3 digits:".to_string(),
        || largest_palindrome_with_two_n_digit_factors(3).unwrap(),
        4,
    );

    dbg_bench(
        "4 digits:".to_string(),
        || largest_palindrome_with_two_n_digit_factors(4).unwrap(),
        4,
    );
}

fn largest_palindrome_with_two_n_digit_factors(n: u8) -> Result<u64, String> {
    let (start, end): (usize, usize) = match n {
        0 => return Err("Factors digit count cannot be 0".to_string()),
        1..=6 => {
            let end = 10_u32.pow(n as u32) - 1;
            let start = end.div_ceil(10);
            (start as usize, end as usize)
        }
        _ => return Err("Input to large".to_string()),
    };

    let palindrome = (start..=end).rfold(0, |max, i| {
        (start..=i)
            .rev()
            .map(|j| j as u64 * i as u64)
            .find(|&p| p > max && is_palindrome(p))
            .unwrap_or(max)
    });

    match palindrome {
        ..=0 => Err("No palindrome found".to_string()),
        _ => Ok(palindrome),
    }
}

fn is_palindrome(n: u64) -> bool {
    let number = n.to_string();
    number.chars().rev().collect::<String>() == number
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_palindrome() {
        assert!(is_palindrome(9009));
        assert!(!is_palindrome(9008));
    }

    #[test]
    fn correct_largest_palindrome_product() {
        assert_eq!(
            9009,
            largest_palindrome_with_two_n_digit_factors(2).unwrap()
        );
    }
}
