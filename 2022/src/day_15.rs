use itertools::Itertools;

pub(crate) fn day_15_1(input: &str, row: isize) -> usize {
    // Parse all sensors and beacons
    let circles = input
        .split(['=', ',', ' ', ':', '\n'])
        .filter_map(|part| part.parse::<isize>().ok())
        .tuples()
        .map(|(sensor_x, sensor_y, beacon_x, beacon_y)| {
            let distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
            let min_x = sensor_x - distance;
            let max_x = sensor_x + distance;
            (
                sensor_x, sensor_y, beacon_x, beacon_y, distance, min_x, max_x,
            )
        })
        .collect::<Vec<_>>();

    // Beacons have to be in this range
    let min_x = *circles
        .iter()
        .map(|(_, _, _, _, _, min_x, _)| min_x)
        .min()
        .unwrap();
    let max_x = *circles
        .iter()
        .map(|(_, _, _, _, _, _, max_x)| max_x)
        .max()
        .unwrap();

    // For all points on the row, check if they are contained in any sensor ranges (do not count the beacons)
    let mut counter = 0;
    for x in min_x..=max_x {
        if circles
            .iter()
            .any(|(sensor_x, sensor_y, beacon_x, beacon_y, distance, _, _)| {
                !(*beacon_x == x && *beacon_y == row)
                    && (sensor_x - x).abs() + (sensor_y - row).abs() <= *distance
            })
        {
            counter += 1;
        }
    }
    counter
}

pub(crate) fn day_15_2(input: &str, max_search: isize) -> isize {
    // Parse all sensors and beacons
    let circles = input
        .split(['=', ',', ' ', ':', '\n'])
        .filter_map(|part| part.parse::<isize>().ok())
        .tuples()
        .map(|(sensor_x, sensor_y, beacon_x, beacon_y)| {
            let distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
            (sensor_x, sensor_y, beacon_x, beacon_y, distance)
        })
        .collect::<Vec<_>>();

    // Collect all edges of sensors
    let edges = circles
        .iter()
        .map(|(x, y, _, _, distance)| {
            (*x..=*x + distance)
                .zip((*y..=*y + distance).rev())
                .chain((*x..*x + distance).zip(*y - distance..*y))
                .chain((*x - distance..*x).zip(*y..*y + distance))
                .chain((*x - distance + 1..*x).zip((*y - distance..*y).rev()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<(isize, isize)>>();

    // For each edge, check if neighbors are covered
    let mut found = 0;
    edges.iter().find(|(x, y)| {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .any(|(diff_x, diff_y)| {
                let x = x + diff_x;
                let y = y + diff_y;
                circles
                    .iter()
                    .all(|(sensor_x, sensor_y, beacon_x, beacon_y, distance)| {
                        if (x < 0 || x > max_search) || (y < 0 || y > max_search) {
                            false // Outside of search scope
                        } else if beacon_x == &x && beacon_y == &y {
                            false // On a beacon
                        } else if sensor_x == &x && sensor_y == &y {
                            false // On a sensor
                        } else if (sensor_x - x).abs() + (sensor_y - y).abs() <= *distance {
                            false // Inside sensor range
                        } else {
                            found = (x * 4000000) + y;
                            true
                        }
                    })
            })
    });
    found
}

#[test]
fn test_day_15_1() {
    let input = String::from_utf8_lossy(include_bytes!("../test_input/day_15.txt"));
    assert_eq!(day_15_1(&input, 10), 26);
}

#[test]
fn test_day_15_2() {
    let input = String::from_utf8_lossy(include_bytes!("../test_input/day_15.txt"));
    assert_eq!(day_15_2(&input, 20), 56000011);
}
