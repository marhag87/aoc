use std::collections::HashMap;

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }
}

pub(crate) fn day_1_1(input: &str) -> i64 {
    use Direction::*;
    let mut x = 0;
    let mut y = 0;
    let mut dir = North;
    input.lines().for_each(|line| {
        line.split(", ").for_each(|instruction| {
            let mut chars = instruction.chars();
            let turn = chars.next().expect("should have a turn");
            let length = chars.as_str().parse::<i64>().expect("should be a number");
            dir = match turn {
                'R' => dir.right(),
                'L' => dir.left(),
                _ => panic!("invalid direction"),
            };
            (x, y) = match dir {
                North => (x, y + length),
                East => (x + length, y),
                South => (x, y - length),
                West => (x - length, y),
            }
        });
    });
    x.abs() + y.abs()
}

pub(crate) fn day_1_2(input: &str) -> i64 {
    use Direction::*;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut pos: HashMap<(i64, i64), i32> = HashMap::new();
    let mut dir = North;
    input
        .lines()
        .find_map(|line| {
            Some(
                line.split(", ")
                    .find_map(|instruction| {
                        let mut chars = instruction.chars();
                        let turn = chars.next().expect("should have a turn");
                        let length = chars.as_str().parse::<i64>().expect("should be a number");
                        dir = match turn {
                            'R' => dir.right(),
                            'L' => dir.left(),
                            _ => panic!("invalid direction"),
                        };
                        (x, y) = match dir {
                            North => (x, y + length),
                            East => (x + length, y),
                            South => (x, y - length),
                            West => (x - length, y),
                        };
                        if let Some(_) = pos.get(&(x, y)) {
                            Some(x.abs() + y.abs())
                        } else {
                            pos.insert((x, y), 1);
                            None
                        }
                    })
                    .unwrap(),
            )
        })
        .unwrap()
}

#[test]
fn test_day_1_1() {
    let input = "R8, R4, R4, R8";
    assert_eq!(day_1_2(input), 4);
}
