#[allow(dead_code)]
pub fn problem_006() -> i64 {
    let sum_of_squares: i64 = (1..=100).map(|x| x * x).sum();
    let square_of_sum = (1..=100).sum::<i64>().pow(2);
    square_of_sum - sum_of_squares
}
