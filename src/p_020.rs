fn explode_carry(mut digits: Vec<u64>) -> Vec<u64> {
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

pub fn problem_020() -> u64 {
    let mut number: Vec<u64> = vec![1];

    for n in 1u64..=100 {
        for e in &mut number {
            *e *= n;
        }
        number = explode_carry(number);
    }
    number.iter().sum::<u64>()
}
