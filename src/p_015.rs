fn comb(n: i128, k: i128) -> i128 {
    if k == 0 {
        1
    } else if k > n / 2 {
        comb(n, n - k)
    } else {
        n * comb(n - 1, k - 1) / k
    }
}

#[allow(dead_code)]
pub fn problem_015() -> i128 {
    let k = 20;
    let n = k * 2;
    comb(n, k)
}
