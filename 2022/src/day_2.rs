pub(crate) fn day_2_1(input: String) -> u64 {
    input
        .split("\n")
        .map(|line| match line {
            // Rock: 1
            "A X" => 4, // Rock
            "B X" => 1, // Paper
            "C X" => 7, // Scissor
            // Paper: 2
            "A Y" => 8, // Rock
            "B Y" => 5, // Paper
            "C Y" => 2, // Scissor
            // Scissor: 3
            "A Z" => 3, // Rock
            "B Z" => 9, // Paper
            "C Z" => 6, // Scissor
            _ => 0,
        })
        .sum()
}

pub(crate) fn day_2_2(input: String) -> u64 {
    input
        .split("\n")
        .map(|line| match line {
            // Loss: 0
            "A X" => 3, // Rock, Scissor
            "B X" => 1, // Paper, Rock
            "C X" => 2, // Scissor, Paper
            // Draw: 3
            "A Y" => 4, // Rock, Rock
            "B Y" => 5, // Paper, Paper
            "C Y" => 6, // Scissor, Scissor
            // Win: 6
            "A Z" => 8, // Rock, Paper
            "B Z" => 9, // Paper, Scissor
            "C Z" => 7, // Scissor, Rock
            _ => 0,
        })
        .sum()
}

#[test]
fn test_example_part_1() {
    let input = "A Y\nB X\nC Z".to_string();
    assert_eq!(day_2_1(input), 15);
}

#[test]
fn test_example_part_2() {
    let input = "A Y\nB X\nC Z".to_string();
    assert_eq!(day_2_2(input), 12);
}
