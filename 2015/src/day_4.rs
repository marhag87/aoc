use md5;

pub(crate) fn day_4_1(input: &str) -> u64 {
    let mut n = 1;
    while !format!("{:?}", md5::compute(format!("{}{}", input, n).as_bytes())).starts_with("00000")
    {
        n += 1;
    }
    n
}

pub(crate) fn day_4_2(input: &str) -> u64 {
    let mut n = 1;
    while !format!("{:?}", md5::compute(format!("{}{}", input, n).as_bytes())).starts_with("000000")
    {
        n += 1;
    }
    n
}

#[test]
fn test_day_4_1() {
    assert_eq!(day_4_1("abcdef"), 609043)
}
