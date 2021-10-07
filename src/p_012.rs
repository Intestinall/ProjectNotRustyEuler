pub fn problem_012() -> u64 {
    let mut n: u64 = 1;
    let mut incrementer = 2;
    loop {
        n += incrementer;
        incrementer += 1;

        if n % 2 != 0 {
            continue;
        }

        let n_sqrt = (n as f64).sqrt() as u64;
        let count = (2..=n_sqrt)
            .map(|i| if n % i == 0 { 2 } else { 0 })
            .sum::<u64>()
            + 2;
        if count > 500 {
            return n;
        }
    }
}
