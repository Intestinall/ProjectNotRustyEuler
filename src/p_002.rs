#[allow(dead_code)]
pub fn problem_002() -> i64 {
    let mut a = 1;
    let mut b = 2;

    let mut sum = b;

    while b < 4_000_000 {
        let c = a + b;
        if c % 2 == 0 {
            sum += c;
        }
        a = b;
        b = c;
    }
    sum
}
