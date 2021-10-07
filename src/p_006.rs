pub fn problem_006() -> u64 {
    let sum_of_squares: u64 = (1..=100).map(|x| x * x).sum();
    let square_of_sum = (1..=100).sum::<u64>().pow(2);
    square_of_sum - sum_of_squares
}
