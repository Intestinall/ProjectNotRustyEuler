#[allow(dead_code)]
pub fn problem_012() -> i64 {
    let mut n: i64 = 1;
    let mut incrementer = 2;
    loop {
        n += incrementer;
        incrementer += 1;

        if n % 2 != 0 {
            continue;
        }

        let n_sqrt = (n as f64).sqrt() as i64;
        let count = (2..=n_sqrt)
            .map(|i| if n % i == 0 { 2 } else { 0 })
            .sum::<i64>()
            + 2;
        if count > 500 {
            return n;
        }
    }
}
