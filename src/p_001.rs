#[allow(dead_code)]
pub fn problem_001() -> i128 {
    (0..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}
