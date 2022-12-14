use itertools::Itertools;
use std::{cmp::max, cmp::min, collections::HashMap};

pub(crate) fn day_14(input: &str) -> (usize, usize) {
    // Generate map
    let mut map: HashMap<(usize, usize), bool> = HashMap::new();
    input.lines().for_each(|line| {
        line.split(" -> ")
            .map(|coords| {
                (
                    coords[0..3].parse::<usize>().unwrap(),
                    coords[4..].parse::<usize>().unwrap(),
                )
            })
            .tuple_windows()
            .for_each(|((start_x, start_y), (end_x, end_y))| {
                for x in min(start_x, end_x)..=max(start_x, end_x) {
                    for y in min(start_y, end_y)..=max(start_y, end_y) {
                        map.insert((x, y), true);
                    }
                }
            })
    });

    // Fill with sand
    let mut path: Vec<(usize, usize)> = vec![(500, 0)];
    let mut filled_source = false;
    let max_y = *map.keys().map(|(_, y)| y).max().unwrap();
    let floor_y = max_y + 2;
    let mut found_void = false;
    let mut p1_sand = 0;

    while !filled_source {
        if let Some((x, y)) = path.pop() {
            if map.get(&(x, y + 1)).is_some() {
                // Tile down is not air, try down+left
                if map.get(&(x - 1, y + 1)).is_some() {
                    // Tile down+left is not air, try down+right
                    if map.get(&(x + 1, y + 1)).is_some() {
                        // Tile down+right is taken, settle
                        map.insert((x, y), false);
                        if !found_void {
                            p1_sand += 1;
                        };
                    } else {
                        // Tile down+right is available, settle if it's not the floor
                        if y + 1 != floor_y {
                            path.push((x, y));
                            path.push((x + 1, y + 1));
                        }
                    }
                } else {
                    // Tile down+left is available, settle if it's not the floor
                    if y + 1 != floor_y {
                        path.push((x, y));
                        path.push((x - 1, y + 1));
                    }
                }
            } else {
                // Tile down is available, settle if it's not the floor
                if y + 1 == floor_y {
                    map.insert((x, y), false);
                } else {
                    // Tile down is air, fall down
                    path.push((x, y));
                    path.push((x, y + 1));
                }
            }
            // Stop part 1 if we've fallen into the void
            if (y + 1) > max_y {
                found_void = true;
            }
            // Stop part 2 if we've reached the source block
            if let Some(false) = map.get(&(500, 0)) {
                filled_source = true;
            }
        }
    }

    (p1_sand, map.into_iter().filter(|(_, v)| !v).count())
}

#[test]
fn test_day_14_1() {
    assert_eq!(
        day_14("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9"),
        (24, 93)
    );
}
