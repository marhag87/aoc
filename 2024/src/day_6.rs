use std::collections::HashSet;

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_6.txt"));
    let first = day_6_1(&input);
    println!("{:?}", first);
    let second = day_6_2(&input);
    println!("{:?}", second);
}

fn day_6_1(input: &str) -> usize {
    let mut map = parse(input);
    loop {
        let (more, _) = walk(&mut map);
        if !more {
            break;
        }
    }
    map.map
        .into_iter()
        .map(|line| {
            line.into_iter()
                .filter(|position| position == &Position::Visited)
                .count()
        })
        .sum::<usize>()
        + 1
}

fn day_6_2(input: &str) -> usize {
    let map = parse(input);
    map.map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, position)| {
                    if position == &Position::Free {
                        let mut map_mutation = map.clone();
                        map_mutation.map[y][x] = Position::Obstruction;

                        let (_, loops) = blargh(&mut map_mutation);
                        if loops {
                            1
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn blargh(map: &mut Map) -> (bool, bool) {
    loop {
        let (more, loops) = walk(map);
        if loops {
            return (false, true);
        }
        if !more {
            return (false, false);
        }
    }
}

fn parse(input: &str) -> Map {
    let mut guard_x = 0;
    let mut guard_y = 0;
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, character)| match character {
                    '.' => Position::Free,
                    '#' => Position::Obstruction,
                    '^' => {
                        guard_x = x;
                        guard_y = y;
                        Position::Guard(Direction::Up)
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<Position>>()
        })
        .collect::<Vec<Vec<Position>>>();
    let width = map.first().unwrap().len() - 1;
    let height = map.len() - 1;
    Map {
        map,
        x: guard_x,
        y: guard_y,
        width,
        height,
        visited: HashSet::new(),
    }
}

fn walk(map: &mut Map) -> (bool, bool) {
    if let Some(&Position::Guard(direction)) = map.map.get(map.y).and_then(|row| row.get(map.x)) {
        let visited = Visited {
            x: map.x,
            y: map.y,
            direction,
        };
        if map.visited.contains(&visited) {
            return (false, true);
        } else {
            map.visited.insert(visited);
        }
        let (next_x, next_y) = match direction {
            Direction::Up => {
                if map.y == 0 {
                    return (false, false);
                }
                (map.x, map.y - 1)
            }
            Direction::Down => {
                if map.y == map.height {
                    return (false, false);
                }
                (map.x, map.y + 1)
            }
            Direction::Right => {
                if map.x == map.width {
                    return (false, false);
                }
                (map.x + 1, map.y)
            }
            Direction::Left => {
                if map.x == 0 {
                    return (false, false);
                }
                (map.x - 1, map.y)
            }
        };
        if let Some(position) = map.map.get(next_y).and_then(|row| row.get(next_x)) {
            match position {
                Position::Obstruction => {
                    map.map[map.y][map.x] = Position::Guard(direction.rotate());
                }
                Position::Free | Position::Visited => {
                    map.map[next_y][next_x] = Position::Guard(direction);
                    map.map[map.y][map.x] = Position::Visited;
                    map.x = next_x;
                    map.y = next_y;
                }
                Position::Guard(_) => unreachable!(),
            }
        }
    }
    (true, false)
}

#[derive(Clone)]
struct Map {
    map: Vec<Vec<Position>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    visited: HashSet<Visited>,
}

#[derive(Copy, Clone, PartialEq)]
enum Position {
    Free,
    Obstruction,
    Visited,
    Guard(Direction),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Visited {
    x: usize,
    y: usize,
    direction: Direction,
}

#[test]
fn test_day_6_1() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    assert_eq!(day_6_1(input), 41);
}

#[test]
fn test_day_6_2() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    assert_eq!(day_6_2(input), 6);
}

#[test]
fn test_day_6_1_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_6.txt"));
    assert_eq!(day_6_1(&input), 5242);
}

// Brute-force, test takes too long
// #[test]
// fn test_day_6_2_answer() {
//     let input = String::from_utf8_lossy(include_bytes!("../input/day_6.txt"));
//     assert_eq!(day_6_2(&input), 1424);
// }
