use itertools::Itertools;

pub(crate) fn day_3_1(input: &str) -> usize {
    let mut houses = vec![(0, 0)];
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    input.chars().for_each(|char| {
        match char {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => panic!("unexpected input"),
        };
        houses.push((x, y));
    });

    houses.iter().unique().count()
}

pub(crate) fn day_3_2(input: &str) -> usize {
    let mut houses = vec![(0, 0)];
    let mut santa_x: i64 = 0;
    let mut santa_y: i64 = 0;
    let mut robot_x: i64 = 0;
    let mut robot_y: i64 = 0;
    let mut n = 0;
    input.chars().for_each(|char| {
        match (char, n % 2) {
            ('^', 0) => santa_y += 1,
            ('v', 0) => santa_y -= 1,
            ('>', 0) => santa_x += 1,
            ('<', 0) => santa_x -= 1,
            ('^', 1) => robot_y += 1,
            ('v', 1) => robot_y -= 1,
            ('>', 1) => robot_x += 1,
            ('<', 1) => robot_x -= 1,
            _ => panic!("unexpected input"),
        };
        if n % 2 == 0 {
            houses.push((santa_x, santa_y));
        } else {
            houses.push((robot_x, robot_y));
        }
        n += 1;
    });

    houses.iter().unique().count()
}

#[test]
fn test_day_3_1() {
    assert_eq!(day_3_1(">"), 2);
    assert_eq!(day_3_1("^>v<"), 4);
    assert_eq!(day_3_1("^v^v^v^v^v"), 2);
}

#[test]
fn test_day_3_2() {
    assert_eq!(day_3_2("^v"), 3);
    assert_eq!(day_3_2("^>v<"), 3);
    assert_eq!(day_3_2("^v^v^v^v^v"), 11);
}
