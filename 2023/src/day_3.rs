fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_3.txt"));
    let first = day_3_1(&input);
    println!("{:?}", first);
    // let second = day_2_2(&input);
    // println!("{:?}", second);
}

fn day_3_1(input: &str) -> usize {
    let mut symbols = vec![];
    let _height = input.lines().collect::<Vec<&str>>().len() - 1;
    let _width = input.lines().next().unwrap().len() - 1;
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' && !char.is_ascii_digit() {
                symbols.push((x, y));
            }
        }
    }
    println!("{:?}", symbols);

    1
}

#[test]
fn test_day_3_1() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    assert_eq!(day_3_1(input), 4361);
}
