use num_bigint::BigUint;
use num_traits::One;
use std::mem::replace;

#[allow(dead_code)]
pub fn problem_025() -> u64 {
    let mut index = 2;
    let mut f0: BigUint = One::one();
    let mut f1: BigUint = One::one();

    loop {
        let f2 = f0 + &f1;

        f0 = replace(&mut f1, f2);

        if f0.to_string().len() >= 1000 {
            return index;
        }
        index += 1;
    }
}
