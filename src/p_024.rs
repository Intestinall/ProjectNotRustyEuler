use itertools::Itertools;

#[allow(dead_code)]
pub fn problem_024() -> i64 {
    let n = 9;
    let millionth_permutation = (0..=n).permutations(n + 1).nth(999_999).unwrap();
    millionth_permutation
        .iter()
        .map(|x| x.to_string())
        .join("")
        .parse::<i64>()
        .unwrap()
}
