use itertools::Itertools;

pub(crate) fn day_8(input: &str) -> (usize, usize) {
    let matrix_size = input.lines().count();
    let mut pos = vec![vec![0; matrix_size]; matrix_size];

    // Populate matrix
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            let n = usize::try_from(char.to_digit(10).expect("should be a number"))
                .expect("should be convertable");
            pos[y][x] = n;
        });
    });

    // Find visibility
    let part_1 = pos
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, num)| {
                    let left = (0..*x)
                        .map(|inner_x| pos[y][inner_x])
                        .collect::<Vec<usize>>();
                    let right = (x + 1..matrix_size)
                        .map(|inner_x| pos[y][inner_x])
                        .collect::<Vec<usize>>();
                    let up = (0..y)
                        .map(|inner_y| pos[inner_y][*x])
                        .collect::<Vec<usize>>();
                    let down = (y + 1..matrix_size)
                        .map(|inner_y| pos[inner_y][*x])
                        .collect::<Vec<usize>>();
                    [left, right, up, down].into_iter().any(|line_to_edge| {
                        let num_same = line_to_edge.iter().filter(|n| n == num).count();
                        if let Some(max) = line_to_edge.iter().max() {
                            max < num && num_same == 0
                        } else {
                            true // We're at an edge
                        }
                    })
                })
                .count()
        })
        .sum();

    // Find scenic score
    let part_2 = pos
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, num)| {
                    let left = (0..x)
                        .rev()
                        .map(|inner_x| pos[y][inner_x])
                        .collect::<Vec<usize>>();
                    let right = (x + 1..matrix_size)
                        .map(|inner_x| pos[y][inner_x])
                        .collect::<Vec<usize>>();
                    let up = (0..y)
                        .rev()
                        .map(|inner_y| pos[inner_y][x])
                        .collect::<Vec<usize>>();
                    let down = (y + 1..matrix_size)
                        .map(|inner_y| pos[inner_y][x])
                        .collect::<Vec<usize>>();
                    [left, right, up, down]
                        .into_iter()
                        .map(|line_to_edge| {
                            if let Some(pos) =
                                line_to_edge.iter().enumerate().find_map(|(pos, tree)| {
                                    if tree >= num {
                                        Some(pos + 1)
                                    } else {
                                        None
                                    }
                                })
                            {
                                pos
                            } else {
                                line_to_edge.len()
                            }
                        })
                        .tuple_windows()
                        .map(|(a, b, c, d)| a * b * c * d)
                        .sum()
                })
                .max()
                .expect("should have a max")
        })
        .max()
        .expect("should have a max");

    (part_1, part_2)
}

#[test]
fn test_day_8() {
    let (part_1, part_2) = day_8("30373\n25512\n65332\n33549\n35390");
    assert_eq!(part_1, 21);
    assert_eq!(part_2, 8);
}
