fn comb(n: i64, k: i64) -> i64 {
    if k == 0 {
        1
    } else if k > n / 2 {
        comb(n, n - k)
    } else {
        n * comb(n - 1, k - 1) / k
    }
}

#[allow(dead_code)]
pub fn problem_015() -> i64 {
    let k = 20;
    let n = k * 2;
    comb(n, k)
}
