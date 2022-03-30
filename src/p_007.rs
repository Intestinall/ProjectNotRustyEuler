fn is_prime(n: i64) -> bool {
    let n_int_sqrt = (n as f64).sqrt() as i64;

    for i in (3..=n_int_sqrt).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
pub fn problem_007() -> i64 {
    let mut prime_count = 4;
    let mut n = 11;

    while prime_count < 10001 {
        if is_prime(n) {
            prime_count += 1
        }
        n += 2;
    }
    n - 2
}
