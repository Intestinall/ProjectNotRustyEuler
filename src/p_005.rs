#[allow(dead_code)]
pub fn problem_005() -> u64 {
    let possible_divisors = [3, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    let mut n = 40;

    'outer: loop {
        for i in possible_divisors {
            if n % i != 0 {
                n += 20;
                continue 'outer;
            }
        }
        return n;
    }
}
