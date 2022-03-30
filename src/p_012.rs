#[allow(dead_code)]
pub fn problem_012() -> i128 {
    let mut n: i128 = 1;
    let mut incrementer = 2;
    loop {
        n += incrementer;
        incrementer += 1;

        if n % 2 != 0 {
            continue;
        }

        let n_sqrt = (n as f64).sqrt() as i128;
        let count = (2..=n_sqrt)
            .map(|i| if n % i == 0 { 2 } else { 0 })
            .sum::<i128>()
            + 2;
        if count > 500 {
            return n;
        }
    }
}
