/// Gives an impression of time-to-run by running the given closure
/// `count` times and returning the total and per-run average duration.
/// Results are printed to std_out. `title` is used as a recognizable
/// name for the function under test when printing the result.
pub fn dbg_bench<T: std::fmt::Debug>(title: String, func: fn() -> T, count: u8) {
    let now = std::time::Instant::now();
    for _ in 1..count {
        func();
    }
    let result = func();
    let elapsed = now.elapsed();

    println!(
        "{title} -> result: {:?}, runs: {count}, total duration: {:.4?}, avg duration: {:.4?}",
        result,
        elapsed,
        elapsed / count as u32
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
