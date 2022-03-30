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

#[allow(dead_code)]
pub fn problem_020() -> i128 {
    let mut number: Vec<i128> = vec![1];

    for n in 1i128..=100 {
        for e in &mut number {
            *e *= n;
        }
        number = explode_carry(number);
    }
    number.iter().sum::<i128>()
}
