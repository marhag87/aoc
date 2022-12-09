use itertools::Itertools;

fn new_tail_pos(head_x: &isize, head_y: &isize, tail_x: &isize, tail_y: &isize) -> (isize, isize) {
    let diff_x = head_x - tail_x;
    let diff_y = head_y - tail_y;
    let (x, y) = match (diff_x, diff_y) {
        (0, 0) | (1, 0) | (0, 1) | (-1, 0) | (0, -1) | (1, 1) | (-1, -1) | (1, -1) | (-1, 1) => {
            (0, 0)
        }
        (2, 0) => (1, 0),
        (-2, 0) => (-1, 0),
        (0, 2) => (0, 1),
        (0, -2) => (0, -1),
        (-2, 1) | (-1, 2) => (-1, 1),
        (2, 1) | (1, 2) => (1, 1),
        (2, -1) | (1, -2) => (1, -1),
        (-1, -2) | (-2, -1) => (-1, -1),
        (2, 2) => (1, 1),
        (2, -2) => (1, -1),
        (-2, 2) => (-1, 1),
        (-2, -2) => (-1, -1),
        _ => {
            println!(
                "({} {}), ({}, {}), ({}, {})",
                head_x, head_y, tail_x, tail_y, diff_x, diff_y
            );
            panic!("invalid position");
        }
    };
    (tail_x + x, tail_y + y)
}

fn day_9(input: &str, rope_length: usize) -> usize {
    let mut rope = vec![(0, 0); rope_length];
    let mut visited = vec![(0, 0)];
    input.lines().for_each(|line| {
        let mut split = line.split_ascii_whitespace();
        let direction = split.next().expect("should have a direction");
        let length = split
            .next()
            .expect("should have a length")
            .parse::<isize>()
            .expect("should be a number");
        let (x, y) = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => panic!("invalid direction"),
        };

        (0..length).for_each(|_| {
            let mut last_x = 0;
            let mut last_y = 0;
            let mut n = 0;
            rope.iter_mut().for_each(|(knot_x, knot_y)| {
                if n == 0 {
                    // First knot, set directly
                    *knot_x += x;
                    *knot_y += y;
                } else {
                    // Get new pos based on previous knot
                    (*knot_x, *knot_y) = new_tail_pos(&last_x, &last_y, &knot_x, &knot_y);
                };
                if n == rope_length - 1 {
                    visited.push((knot_x.clone(), knot_y.clone()));
                };
                last_x = knot_x.clone();
                last_y = knot_y.clone();
                n += 1;
            });
        });
    });
    visited.iter().unique().count()
}

pub(crate) fn day_9_1(input: &str) -> usize {
    day_9(input, 2)
}

pub(crate) fn day_9_2(input: &str) -> usize {
    day_9(input, 10)
}

#[test]
fn test_new_tail_pos() {
    assert_eq!(new_tail_pos(&0, &0, &0, &0), (0, 0));
    assert_eq!(new_tail_pos(&1, &0, &0, &0), (0, 0));
    assert_eq!(new_tail_pos(&2, &0, &0, &0), (1, 0));
    assert_eq!(new_tail_pos(&2, &0, &1, &0), (1, 0));
    assert_eq!(new_tail_pos(&1, &1, &0, &0), (0, 0));
    assert_eq!(new_tail_pos(&-1, &-1, &0, &0), (0, 0));
    assert_eq!(new_tail_pos(&1, &2, &0, &0), (1, 1));
}

#[test]
fn test_day_9_1() {
    assert_eq!(day_9_1("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"), 13)
}

#[test]
fn test_day_9_2() {
    assert_eq!(day_9_2("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20"), 36);
}
