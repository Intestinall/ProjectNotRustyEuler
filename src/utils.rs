use colored::*;
use std::time::Instant;

pub fn run_problem(problem_id: u16, f: fn() -> u64, answer: u64) {
    let now = Instant::now();
    let result = f();
    match result == answer {
        true => {
            let elapsed_time = now.elapsed().as_millis();
            println!(
                "[{}] P{:03} => elapsed time: {}ms",
                "SUCCESS".green(),
                problem_id,
                if elapsed_time > 1000 {
                    elapsed_time.to_string().yellow()
                } else {
                    elapsed_time.to_string().bright_blue()
                }
            )
        }
        false => {
            println!(
                "[{}] P{:03} => expected {}, got {}",
                "FAILED".red(),
                problem_id,
                answer,
                result
            )
        }
    };
}
