pub(crate) fn day_4_1(input: String) -> usize {
    input
        .lines()
        .map(|line| scan_fmt!(line, "{d}-{d},{d}-{d}", u64, u64, u64, u64).expect("should match"))
        .filter(|(one, two, three, four)| {
            (one >= three && two <= four) || (three >= one && four <= two)
        })
        .count()
}

pub(crate) fn day_4_2(input: String) -> usize {
    input
        .lines()
        .map(|line| scan_fmt!(line, "{d}-{d},{d}-{d}", u64, u64, u64, u64).expect("should match"))
        .filter(|(one, two, three, four)| {
            (one <= three && three <= two) || (three <= one && one <= four)
        })
        .count()
}
