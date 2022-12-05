#[repr(u64)]
enum Pick {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[repr(u64)]
enum State {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

pub(crate) fn day_2_1(input: &str) -> u64 {
    use Pick::*;
    use State::*;
    input
        .split('\n')
        .map(|line| match line {
            "A X" => Rock as u64 + Draw as u64,
            "B X" => Rock as u64 + Loss as u64,
            "C X" => Rock as u64 + Win as u64,
            "A Y" => Paper as u64 + Win as u64,
            "B Y" => Paper as u64 + Draw as u64,
            "C Y" => Paper as u64 + Loss as u64,
            "A Z" => Scissor as u64 + Loss as u64,
            "B Z" => Scissor as u64 + Win as u64,
            "C Z" => Scissor as u64 + Draw as u64,
            _ => 0,
        })
        .sum()
}

pub(crate) fn day_2_2(input: &str) -> u64 {
    use Pick::*;
    use State::*;
    input
        .split('\n')
        .map(|line| match line {
            "A X" => Loss as u64 + Scissor as u64,
            "B X" => Loss as u64 + Rock as u64,
            "C X" => Loss as u64 + Paper as u64,
            "A Y" => Draw as u64 + Rock as u64,
            "B Y" => Draw as u64 + Paper as u64,
            "C Y" => Draw as u64 + Scissor as u64,
            "A Z" => Win as u64 + Paper as u64,
            "B Z" => Win as u64 + Scissor as u64,
            "C Z" => Win as u64 + Rock as u64,
            _ => 0,
        })
        .sum()
}

#[test]
fn test_example_part_1() {
    let input = "A Y\nB X\nC Z";
    assert_eq!(day_2_1(input), 15);
}

#[test]
fn test_example_part_2() {
    let input = "A Y\nB X\nC Z";
    assert_eq!(day_2_2(input), 12);
}
