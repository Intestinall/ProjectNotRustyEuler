use std::collections::HashSet;

fn get_all_divisors(n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    v.push(1);

    let sqrt_n = (n as f64).sqrt() as u64;
    for m in 2..=sqrt_n {
        if n % m == 0 {
            v.push(m);
            let c = n / m;
            if c != m {
                v.push(c);
            }
        }
    }
    v
}

#[allow(dead_code)]
pub fn problem_021() -> u64 {
    let mut seen = HashSet::new();

    for a in 1..10000 {
        let d_a = get_all_divisors(a).iter().sum();
        let b = d_a;
        let d_b = get_all_divisors(b).iter().sum();

        if a == d_b && b == d_a && a != b && !seen.contains(&a) {
            seen.insert(a);
            seen.insert(b);
        }
    }

    seen.iter().sum()
}
