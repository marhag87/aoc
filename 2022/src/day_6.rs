use itertools::Itertools;

fn find_window(input: &str, window_size: usize) -> usize {
    if let Some((n, _)) = input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find(|(_, window)| window.iter().all_unique())
    {
        n + window_size
    } else {
        panic!("No solution found")
    }
}

pub(crate) fn day_6_1(input: &str) -> usize {
    find_window(input, 4)
}
pub(crate) fn day_6_2(input: &str) -> usize {
    find_window(input, 14)
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
