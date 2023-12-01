fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_1.txt"));
    let first = day_1_1(&input);
    println!("{:?}", first);
    let second = day_1_2(&input);
    println!("{:?}", second);
}

fn day_1_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find(|char| char.is_numeric()).unwrap();
            let last = line.chars().rev().find(|char| char.is_numeric()).unwrap();
            let num = format!("{first}{last}");
            num.parse::<u32>().unwrap()
        })
        .sum()
}

fn find(
    input: &str,
    search: &str,
    num: usize,
    li: &mut usize,
    first: &mut usize,
    highest_index: &mut usize,
    last: &mut usize,
) {
    input.match_indices(search).for_each(|(index, _)| {
        if index < *li {
            *li = index;
            *first = num;
        }
        if index >= *highest_index {
            *highest_index = index;
            *last = num;
        }
    });
}

fn day_1_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut li = usize::MAX; // Lowest index
            let mut l = usize::MAX; // First number
            let mut hi = usize::MIN; // Highest index
            let mut h = usize::MIN; // Last number

            find(line, "1", 1, &mut li, &mut l, &mut hi, &mut h);
            find(line, "2", 2, &mut li, &mut l, &mut hi, &mut h);
            find(line, "3", 3, &mut li, &mut l, &mut hi, &mut h);
            find(line, "4", 4, &mut li, &mut l, &mut hi, &mut h);
            find(line, "5", 5, &mut li, &mut l, &mut hi, &mut h);
            find(line, "6", 6, &mut li, &mut l, &mut hi, &mut h);
            find(line, "7", 7, &mut li, &mut l, &mut hi, &mut h);
            find(line, "8", 8, &mut li, &mut l, &mut hi, &mut h);
            find(line, "9", 9, &mut li, &mut l, &mut hi, &mut h);
            find(line, "one", 1, &mut li, &mut l, &mut hi, &mut h);
            find(line, "two", 2, &mut li, &mut l, &mut hi, &mut h);
            find(line, "three", 3, &mut li, &mut l, &mut hi, &mut h);
            find(line, "four", 4, &mut li, &mut l, &mut hi, &mut h);
            find(line, "five", 5, &mut li, &mut l, &mut hi, &mut h);
            find(line, "six", 6, &mut li, &mut l, &mut hi, &mut h);
            find(line, "seven", 7, &mut li, &mut l, &mut hi, &mut h);
            find(line, "eight", 8, &mut li, &mut l, &mut hi, &mut h);
            find(line, "nine", 9, &mut li, &mut l, &mut hi, &mut h);
            let num = format!("{l}{h}");
            num.parse::<u32>().unwrap()
        })
        .sum()
}

#[test]
fn test_example_input_1() {
    let input = r#"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"#;
    assert_eq!(day_1_1(input), 142);
}

#[test]
fn test_example_input_2() {
    let input = r#"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"#;
    assert_eq!(day_1_2(input), 281);
}

#[test]
fn test_extra() {
    let input = r#"oneone"#;
    assert_eq!(day_1_2(input), 11);
    let input = r#"twone"#;
    assert_eq!(day_1_2(input), 21);
    let input = r#"eightwo"#;
    assert_eq!(day_1_2(input), 82);
    let input = r#"nineight"#;
    assert_eq!(day_1_2(input), 98);
    let input = r#"eighthree"#;
    assert_eq!(day_1_2(input), 83);
    let input = r#"nineeight"#;
    assert_eq!(day_1_2(input), 98);
    let input = r#"sevenine"#;
    assert_eq!(day_1_2(input), 79);
    let input = r#"6gqsvsqpzxj"#;
    assert_eq!(day_1_2(input), 66);
}
