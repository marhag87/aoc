use itertools::Itertools;

pub(crate) fn day_6_1(input: &str) -> usize {
    let mut n = 3;
    input.as_bytes().windows(4).any(|window| {
        n += 1;
        window.iter().all_unique()
    });
    n
}

pub(crate) fn day_6_2(input: &str) -> usize {
    let mut n = 13;
    input.as_bytes().windows(14).any(|window| {
        n += 1;
        window.iter().all_unique()
    });
    n
}

#[test]
fn test_day_6_1() {
    assert_eq!(day_6_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(day_6_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(day_6_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(day_6_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

#[test]
fn test_day_6_2() {
    assert_eq!(day_6_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(day_6_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(day_6_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(day_6_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(day_6_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}
