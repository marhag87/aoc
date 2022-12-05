use itertools::Itertools;

pub(crate) fn day_5_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let at_least_three_vowels = line
                .chars()
                .filter(|char| match char {
                    'a' | 'e' | 'i' | 'o' | 'u' => true,
                    _ => false,
                })
                .count()
                >= 3;
            let double_char = line.chars().tuple_windows().any(|(a, b)| a == b);
            let has_forbidden_words = line.contains("ab")
                || line.contains("cd")
                || line.contains("pq")
                || line.contains("xy");
            at_least_three_vowels && double_char && !has_forbidden_words
        })
        .count()
}

pub(crate) fn day_5_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let appears_twice = line
                .chars()
                .tuple_windows()
                .any(|(a, b, c)| line.matches(&format!("{}{}", a, b)).count() >= 2);
            let repeats_with_other_between = line
                .chars()
                .tuple_windows()
                .any(|(a, b, c)| a == c && a != b);
            appears_twice && repeats_with_other_between
        })
        .count()
}
