use std::fs;
use std::path::Path;

#[allow(dead_code)]
pub fn problem_022() -> i64 {
    let path = Path::new("files/p022_names.txt").canonicalize().unwrap();
    let data = fs::read_to_string(path).unwrap();
    let mut string_vec: Vec<String> = data.split(',').map(|s| s.to_owned()).collect();
    string_vec.sort();

    string_vec
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let v = s.trim_matches('"').as_bytes();
            let char_sum = v.iter().map(|n| *n as i64).sum::<i64>() as i64;
            let char_offset = v.len() as i64 * 64;
            (char_sum - char_offset) * ((i as i64) + 1)
        })
        .sum::<i64>()
}
