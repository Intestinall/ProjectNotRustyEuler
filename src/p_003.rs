use std::cmp::max;

fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n == 0 || n == 1 || n % 2 == 0 {
        return false;
    }

    let n_int_sqrt = max((n as f64).sqrt() as u64, 3);

    for i in (3..=n_int_sqrt).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
pub fn problem_003() -> u64 {
    let mut n: u64 = 600851475143;
    let mut found_primes: Vec<u64> = vec![2, 3];

    'outer: while n != 1 {
        for p in &found_primes {
            if n % p == 0 {
                n /= p;
                continue 'outer;
            }
        }

        let mut i = found_primes.last().unwrap() + 2;
        loop {
            if is_prime(i) && n % i == 0 {
                found_primes.push(i);
                n /= i;
                continue 'outer;
            }
            i += 2;
        }
    }
    found_primes.last().unwrap().to_owned()
}
