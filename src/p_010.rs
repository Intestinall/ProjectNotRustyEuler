fn is_prime(n: i128) -> bool {
    let n_int_sqrt = (n as f64).sqrt() as i128;

    for i in (3..=n_int_sqrt).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
pub fn problem_010() -> i128 {
    let s: i128 = (11..2_000_000).step_by(2).filter(|n| is_prime(*n)).sum();
    2 + 3 + 5 + 7 + s
}
