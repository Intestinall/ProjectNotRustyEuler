use crate::common::sieve_of_eratosthenes;


#[allow(dead_code)]
pub fn problem_010() -> i64 {
    sieve_of_eratosthenes(2_000_000).iter().sum()
}
