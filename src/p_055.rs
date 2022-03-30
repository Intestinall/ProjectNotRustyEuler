fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn is_palindromic(a: u128) -> bool {
    let string_a = a.to_string();
    string_a == reverse_string(&string_a)
}

fn reverse(n: u128) -> u128 {
    reverse_string(&n.to_string()).parse().unwrap()
}

#[allow(dead_code)]
pub fn problem_055() -> i64 {
    let mut c = 0;

    'outer: for mut n in 1..10_000 {
        for _ in 1..50 {
            let reversed_n = reverse(n);
            n += reversed_n;
            if is_palindromic(n) {
                continue 'outer;
            }
        }
        c += 1;
    }
    c
}
