pub(crate) fn day_6_1(input: &str) -> usize {
    let mut lights = vec![vec![false; 1000]; 1000];
    input
        .lines()
        .for_each(|line| match line.split_ascii_whitespace().next() {
            Some("turn") => {
                if let Ok((state, start_x, start_y, end_x, end_y)) = scan_fmt!(
                    line,
                    "turn {} {d},{d} through {d},{d}",
                    String,
                    usize,
                    usize,
                    usize,
                    usize
                ) {
                    match state.as_str() {
                        "on" => (start_x..=end_x).into_iter().for_each(|x| {
                            (start_y..=end_y).into_iter().for_each(|y| {
                                lights[x][y] = true;
                            })
                        }),
                        "off" => (start_x..=end_x).into_iter().for_each(|x| {
                            (start_y..=end_y).into_iter().for_each(|y| {
                                lights[x][y] = false;
                            })
                        }),
                        _ => panic!("invalid input"),
                    }
                }
            }
            Some("toggle") => {
                if let Ok((start_x, start_y, end_x, end_y)) = scan_fmt!(
                    line,
                    "toggle {d},{d} through {d},{d}",
                    usize,
                    usize,
                    usize,
                    usize
                ) {
                    (start_x..=end_x).into_iter().for_each(|x| {
                        (start_y..=end_y).into_iter().for_each(|y| {
                            lights[x][y] = !lights[x][y];
                        })
                    })
                }
            }
            _ => panic!("invalid input"),
        });
    lights
        .into_iter()
        .map(|row| row.into_iter().filter(|light| *light).count())
        .sum()
}

pub(crate) fn day_6_2(input: &str) -> usize {
    let mut lights: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    input
        .lines()
        .for_each(|line| match line.split_ascii_whitespace().next() {
            Some("turn") => {
                if let Ok((state, start_x, start_y, end_x, end_y)) = scan_fmt!(
                    line,
                    "turn {} {d},{d} through {d},{d}",
                    String,
                    usize,
                    usize,
                    usize,
                    usize
                ) {
                    match state.as_str() {
                        "on" => (start_x..=end_x).into_iter().for_each(|x| {
                            (start_y..=end_y).into_iter().for_each(|y| {
                                lights[x][y] += 1;
                            })
                        }),
                        "off" => (start_x..=end_x).into_iter().for_each(|x| {
                            (start_y..=end_y).into_iter().for_each(|y| {
                                if lights[x][y] > 0 {
                                    lights[x][y] -= 1;
                                }
                            })
                        }),
                        _ => panic!("invalid input"),
                    }
                }
            }
            Some("toggle") => {
                if let Ok((start_x, start_y, end_x, end_y)) = scan_fmt!(
                    line,
                    "toggle {d},{d} through {d},{d}",
                    usize,
                    usize,
                    usize,
                    usize
                ) {
                    (start_x..=end_x).into_iter().for_each(|x| {
                        (start_y..=end_y).into_iter().for_each(|y| {
                            lights[x][y] += 2;
                        })
                    })
                }
            }
            _ => panic!("invalid input"),
        });
    lights
        .into_iter()
        .map(|row| row.into_iter().sum::<usize>())
        .sum::<usize>()
}
