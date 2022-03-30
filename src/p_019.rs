fn is_leap_year(year: i64) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

#[allow(dead_code)]
pub fn problem_019() -> i64 {
    let mut sunday_count = 0;
    let mut current_week_day = 1;
    let mut month_days: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    for year in 1901..=2000 {
        month_days[1] = if is_leap_year(year) { 29 } else { 28 };

        for days in &month_days {
            if current_week_day == 6 {
                sunday_count += 1
            }
            current_week_day = (current_week_day + *days) % 7;
        }
    }
    sunday_count
}
