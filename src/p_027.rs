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
pub fn problem_027() -> i128 {
    let _v: Vec<i128> = (11..10_000).step_by(2).filter(|n| is_prime(*n)).collect();
    0
}
