pub(crate) fn day_1_1(input: &str) -> (i64, i64) {
    let mut floor = 0;
    let mut position = -1;
    let mut n = 1;
    input.chars().for_each(|char| {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Unknown instruction"),
        };
        if floor < 0 && position == -1 {
            position = n;
        };
        n += 1;
    });
    (floor, position)
}
