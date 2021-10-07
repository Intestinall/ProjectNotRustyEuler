use itertools::Itertools;
use std::cmp::Reverse;

pub fn problem_004() -> u64 {
    let mut vec: Vec<(u64, u64)> = (100..1000).rev().tuple_combinations().collect();
    vec.sort_unstable_by_key(|t| Reverse(t.0 + t.1));

    for (a, b) in vec {
        let r = a * b;
        let string_r = r.to_string();
        if string_r == string_r.chars().rev().collect::<String>() {
            return r;
        }
    }
    unreachable!()
}
