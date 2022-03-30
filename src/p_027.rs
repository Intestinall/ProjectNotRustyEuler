use crate::common::sieve_of_eratosthenes;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn problem_027() -> i64 {
    let all_primes_below_1000: HashSet<i64> = sieve_of_eratosthenes(1000).into_iter().collect();

    let (mut max_consecutive, mut max_a, mut max_b) = (0, 0, 0);
    for a in -1000..1000 {
        for b in -1000..1000 {
            for n in 0i64..1000 {
                let r = n * n + a * n + b;
                if !all_primes_below_1000.contains(&r) {
                    if n > max_consecutive {
                        max_consecutive = n;
                        max_a = a;
                        max_b = b;
                    }
                    break;
                }
            }
        }
    }
    max_a * max_b
}
