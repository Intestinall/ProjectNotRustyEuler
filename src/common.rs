pub fn sieve_of_eratosthenes(n: usize) -> Vec<i128> {
    let mut a: Vec<bool> = vec![true; n as usize];
    a[0] = false;
    a[1] = false;
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 2usize..=sqrt_n {
        if a[i] {
            for j in ((i * i)..n).step_by(i) {
                a[j] = false;
            }
        }
    }
    a.iter()
        .enumerate()
        .filter(|(_, b)| **b == true)
        .map(|(a, _)| a as i128)
        .collect::<Vec<i128>>()
}
