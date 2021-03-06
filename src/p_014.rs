#[allow(dead_code)]
pub fn problem_014() -> i64 {
    let mut max_chain_size: i64 = 0;
    let mut max_n: i64 = 0;

    for n in 3..1_000_000 {
        let mut chain_size: i64 = 1;
        let mut last_chain_n: i64 = n;

        while last_chain_n != 1 {
            if last_chain_n % 2 == 0 {
                last_chain_n /= 2;
            } else {
                last_chain_n = 3 * last_chain_n + 1;
            }
            chain_size += 1;
        }
        if chain_size > max_chain_size {
            max_chain_size = chain_size;
            max_n = n;
        }
    }
    max_n
}
