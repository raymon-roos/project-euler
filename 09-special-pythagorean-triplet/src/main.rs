fn main() {
    dbg_bench(
        "nested loops".to_string(),
        || match gen_pyth_triples_sumed_to(1000) {
            Some((a, b, c)) => a * b * c,
            _ => 0,
        },
        4,
    );

    dbg_bench(
        "iterators".to_string(),
        || match pyth_triples_iterator(1000) {
            Some((a, b, c)) => a * b * c,
            _ => 0,
        },
        4,
    )
}

fn gen_pyth_triples_sumed_to(n: usize) -> Option<(usize, usize, usize)> {
    for a in 1..999 {
        for b in a..999 {
            for c in b..999 {
                if is_pythagorean_triple((a, b, c)) && a + b + c == n {
                    return Some((a, b, c));
                }
            }
        }
    }
    None
}

fn pyth_triples_iterator(n: usize) -> Option<(usize, usize, usize)> {
    (1..999)
        .flat_map(|a| {
            (a..999).flat_map(move |b| {
                (b..999)
                    .filter(move |&c| is_pythagorean_triple((a, b, c)) && a + b + c == n)
                    .map(move |c| (a, b, c))
            })
        })
        .next()
}

fn is_pythagorean_triple((a, b, c): (usize, usize, usize)) -> bool {
    b < c && a < b && a * a + b * b == c * c
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
    fn correct_pythagorean_triple() {
        assert!(is_pythagorean_triple((3, 4, 5)));
        assert!(!is_pythagorean_triple((4, 3, 5)));
        assert!(!is_pythagorean_triple((2, 4, 6)));
    }
}
