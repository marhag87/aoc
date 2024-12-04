fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_4.txt"));
    let first = day_4_1(&input);
    println!("{:?}", first);
    let second = day_4_2(&input);
    println!("{:?}", second);
}

fn day_4_1(input: &str) -> usize {
    let char_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    char_map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, character)| {
                    if *character == 'X' {
                        [
                            walk(&char_map, 3, x, y, Some(true), None),
                            walk(&char_map, 3, x, y, Some(false), None),
                            walk(&char_map, 3, x, y, None, Some(true)),
                            walk(&char_map, 3, x, y, None, Some(false)),
                            walk(&char_map, 3, x, y, Some(true), Some(true)),
                            walk(&char_map, 3, x, y, Some(true), Some(false)),
                            walk(&char_map, 3, x, y, Some(false), Some(true)),
                            walk(&char_map, 3, x, y, Some(false), Some(false)),
                        ]
                        .into_iter()
                        .filter(|word| *word == Some("MAS".to_string()))
                        .count()
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn day_4_2(input: &str) -> usize {
    let char_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let width = char_map.first().unwrap().len() - 1;
    let height = char_map.len() - 1;
    char_map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, character)| {
                    if *x == 0 || *x == width || y == 0 || y == height || **character != 'A' {
                        false
                    } else {
                        let top_left = char_map[y - 1][x - 1];
                        let top_right = char_map[y - 1][x + 1];
                        let bottom_left = char_map[y + 1][x - 1];
                        let bottom_right = char_map[y + 1][x + 1];
                        ((top_left == 'M' && bottom_right == 'S')
                            || (top_left == 'S' && bottom_right == 'M'))
                            && ((top_right == 'M' && bottom_left == 'S')
                                || (top_right == 'S' && bottom_left == 'M'))
                    }
                })
                .count()
        })
        .sum()
}

fn walk(
    char_map: &[Vec<char>],
    length: usize,
    x: usize,
    y: usize,
    horizontal: Option<bool>,
    vertical: Option<bool>,
) -> Option<String> {
    let x = match (horizontal, x) {
        (Some(true), _) => x + 1,
        (Some(false), 0) => return None,
        (Some(false), _) => x - 1,
        (None, _) => x,
    };
    let y = match (vertical, y) {
        (Some(true), _) => y + 1,
        (Some(false), 0) => return None,
        (Some(false), _) => y - 1,
        (None, _) => y,
    };
    let character = if let Some(row) = char_map.get(y) {
        row.get(x)?
    } else {
        return None;
    };
    if length == 1 {
        Some(format!("{}", character))
    } else {
        walk(char_map, length - 1, x, y, horizontal, vertical)
            .map(|next| format!("{}{}", character, next))
    }
}

#[test]
fn test_day_4_1() {
    let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
    assert_eq!(day_4_1(input), 18);
}

#[test]
fn test_day_4_2() {
    let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
    assert_eq!(day_4_2(input), 9);
}

#[test]
fn test_day_4_1_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_4.txt"));
    assert_eq!(day_4_1(&input), 2454);
}

#[test]
fn test_day_4_2_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_4.txt"));
    assert_eq!(day_4_2(&input), 1858);
}
