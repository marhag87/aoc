use std::cmp::{max, min};

pub(crate) fn day_2_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| scan_fmt!(line, "{d}x{d}x{d}", u64, u64, u64).expect("should be parsable"))
        .map(|(l, w, h)| ((l * w), (w * h), (h * l)))
        .map(|(lw, wh, hl)| (2 * lw) + (2 * wh) + (2 * hl) + min(lw, min(wh, hl)))
        .sum()
}

pub(crate) fn day_2_2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| scan_fmt!(line, "{d}x{d}x{d}", u64, u64, u64).expect("should be parsable"))
        .map(|(l, w, h)| {
            let min_side = min(l, min(w, h));
            let second_min_side = max(min(l, w), max(min(w, h), min(h, l)));
            (l * w * h) + (2 * min_side) + (2 * second_min_side)
        })
        .sum()
}

#[test]
fn test_day_2_2() {
    assert_eq!(day_2_2("2x3x4"), 34);
    assert_eq!(day_2_2("1x1x10"), 14);
}
