use std::collections::HashSet;

fn is_abundant(n: &u64) -> bool {
    let n = *n;
    let mut sum = 1;

    let sqrt_n = (n as f64).sqrt() as u64;
    for m in 2..=sqrt_n {
        if n % m == 0 {
            sum += m;
            let c = n / m;
            if c != m {
                sum += c;
            }
        }
        if sum > n {
            return true;
        }
    }
    sum > n
}

#[allow(dead_code)]
pub fn problem_023() -> u64 {
    let h: HashSet<u64> = (2..=28123).filter(is_abundant).collect();
    let mut sum = 0;

    'outer: for n in 1..=28123 {
        for a in h.iter() {
            let n_minus_abundant = n - a;
            if h.contains(&n_minus_abundant) {
                continue 'outer;
            }
        }
        sum += n;
    }
    sum
}
