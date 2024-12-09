use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_8.txt"));
    let first = day_8_1(&input);
    println!("{:?}", first);
    let second = day_8_2(&input);
    println!("{:?}", second);
}

fn day_8_1(input: &str) -> usize {
    let grid = parse(input);
    let width = grid.first().unwrap().len() as isize;
    let height = grid.len() as isize;
    let mut unique_points = HashSet::new();
    beacons(grid).iter().for_each(|(_beacon, points)| {
        points.iter().permutations(2).for_each(|points| {
            let first = points.first().unwrap();
            let second = points.last().unwrap();
            let x = second.x + (second.x - first.x);
            let y = second.y + (second.y - first.y);
            if x >= 0 && x < width && y >= 0 && y < height {
                unique_points.insert((x, y));
            }
        });
    });
    unique_points.len()
}

fn day_8_2(input: &str) -> usize {
    let grid = parse(input);
    let width = grid.first().unwrap().len() as isize;
    let height = grid.len() as isize;
    let mut unique_points = HashSet::new();
    beacons(grid).iter().for_each(|(_beacon, points)| {
        points.iter().permutations(2).for_each(|points| {
            let first = points.first().unwrap();
            let second = points.last().unwrap();
            unique_points.insert((first.x, first.y));
            unique_points.insert((second.x, second.y));
            let x_diff = second.x - first.x;
            let y_diff = second.y - first.y;
            let mut num = 1;
            loop {
                let x = second.x + (x_diff * num);
                let y = second.y + (y_diff * num);
                num += 1;
                if x >= 0 && x < width && y >= 0 && y < height {
                    unique_points.insert((x, y));
                } else {
                    break;
                }
            }
        });
    });
    unique_points.len()
}

fn beacons(grid: Vec<Vec<Point>>) -> HashMap<char, Vec<Point>> {
    let mut beacons = HashMap::new();
    grid.iter().for_each(|row| {
        row.iter().for_each(|point| {
            if let Some(beacon) = point.beacon {
                beacons
                    .entry(beacon)
                    .and_modify(|beacon_list: &mut Vec<Point>| beacon_list.push(*point))
                    .or_insert(vec![*point]);
            }
        })
    });
    beacons
}

fn parse(input: &str) -> Vec<Vec<Point>> {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, character)| {
                    if character != '.' {
                        Point {
                            x: x as isize,
                            y: y as isize,
                            beacon: Some(character),
                        }
                    } else {
                        Point {
                            x: x as isize,
                            y: y as isize,
                            beacon: None,
                        }
                    }
                })
                .collect::<Vec<Point>>()
        })
        .collect()
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
    beacon: Option<char>,
}

#[test]
fn test_day_8_1() {
    let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
    assert_eq!(day_8_1(input), 14);
}

#[test]
fn test_day_8_2() {
    let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
    assert_eq!(day_8_2(input), 34);
}

#[test]
fn test_day_8_1_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_8.txt"));
    assert_eq!(day_8_1(&input), 394);
}

#[test]
fn test_day_8_2_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_8.txt"));
    assert_eq!(day_8_2(&input), 1277);
}
