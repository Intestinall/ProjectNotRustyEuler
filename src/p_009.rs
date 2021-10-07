use itertools::Itertools;

pub fn problem_009() -> u64 {
    let (a, b, c) = (1u64..1000)
        .tuple_combinations::<(_, _, _)>()
        .find(|v: &(u64, u64, u64)| {
            v.0 < v.1
                && v.1 < v.2
                && v.0.pow(2) + v.1.pow(2) == v.2.pow(2)
                && v.0 + v.1 + v.2 == 1000
        })
        .unwrap();
    a * b * c
}
