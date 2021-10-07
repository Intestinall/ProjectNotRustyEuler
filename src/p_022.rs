use std::fs;
use std::path::Path;

pub fn problem_022() -> u64 {
    let path = Path::new("files/p022_names.txt").canonicalize().unwrap();
    let data = fs::read_to_string(path).unwrap();
    let mut string_vec: Vec<String> = data.split(',').map(|s| s.to_owned()).collect();
    string_vec.sort();

    string_vec
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let v = s.trim_matches('"').as_bytes();
            let char_sum = v.iter().map(|n| *n as u64).sum::<u64>() as u64;
            let char_offset = v.len() as u64 * 64;
            (char_sum - char_offset) * ((i as u64) + 1)
        })
        .sum::<u64>()
}
