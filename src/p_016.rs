fn explode_carry(mut digits: Vec<i128>) -> Vec<i128> {
    let max_index = digits.len() - 1;

    for i in 0..digits.len() {
        let n = digits[i];
        let (q, r) = (n / 10, n % 10);

        if q > 0 {
            digits[i] = r;
            if i == max_index {
                digits.push(q);
                let (q, r) = (q / 10, q % 10);
                if q > 0 {
                    digits[i + 1] = r;
                    digits.push(q);
                }
            } else {
                digits[i + 1] += q;
            }
        }
    }
    digits
}

fn big_pow(base_n: i128, p: i128) -> Vec<i128> {
    let mut digits: Vec<i128> = vec![1];

    for _ in 0..p {
        for n in &mut digits {
            *n *= base_n;
        }

        digits = explode_carry(digits);
    }
    digits.reverse();
    digits
}

#[allow(dead_code)]
pub fn problem_016() -> i128 {
    big_pow(2, 1000).iter().sum()
}
